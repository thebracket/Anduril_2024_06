# Large Async Rust Program Structure

> The original question: "Async beyond basic concurrency primitives (e.g. not “what is a mutex/atomic”) but more along the lines of “architecting/designing a large application”

I admit it: I laughed a little, because this is in danger of being a "how long is a piece of string" question. There are so many possible programs, that could be doing so many different things, that it's relatively impossible to answer! Also, Ardan Labs sell my 20 hour "Rust as a Service" video training, in which I spend a lot of time discussing the nuts and bolts of building an Axum-based web service stack, or a Tonic-based gRPC stack. We don't have 20 hours...

## Question 1: How much (if any) async do you want?

Take a good look at what you're trying to do. 

* Anything that is CPU bound or File I/O bound is probably better off in synchronous Rust.
* Anything that is network bound, or spends most of its time waiting for things like databases, is probably better off in async Rust.
* Message-passing architectures are naturally async.

You may well realize "I have some of both". Don't panic, that's *really* common. We're going to assume you have at least some async (from the question).

## Question 2: Async Boundaries

### Just a Little Async

I've been assisting a customer with a design that is mostly synchronous, with just a little async:

On the synchronouse side:

* The core spawns a number of other programs and monitors their output -- one per thread.
* The core also ingests a number of files periodically - also in a thread.

On the asynchronous side:

* Kubernetes APIs are very async, so there's an async system talking to it.
* The system enqueues and submits results from the synchronous side as they are produced. This is async, using an RPC system.
* A "health monitor" tracks the health of each of the threaded processes, and makes it available via a tiny webserver.

So the architecture here:

1. Create some `tokio` MPSSC channels for transmitting to the health monitor and the submission service.
2. Spawn the threads (passing transmitters in) for the synchronous side of things.
3. Launch Tokio with `block_on`. This in turn sets up Tokio tasks:
    1. Submission system keeps a local "send queue" (mutex wrapped)
        * A task is spawned that periodically tries to send the queue, removing items that are sent.
        * A task listens for messages, and submits them to the queue (and handles exit)
    2. The webserver task/future
        * A task retains current service state.
        * A task listens for updates via channel.
        * An Actix server is spawned displaying the state.

### VERY Async

Another customer is using a "modular monolith" style of development, with a view to being able to scale out services if needed in the future (but wisely keeping small while they get started). Their service is mostly a web service.

* An "ingestor" task is spawned.
    * It handles TCP connections containing data to collect (proprietary format)
    * The data is validated and submitted to a database
* A whole bunch of `init` calls are made for child modules.
* A "web" task is spawned.
    * *Many* `Router` objects (Axum) are created from different, separated modules.
    * These are combined with calls to `nest`, to build a single outwards-facing `Router`.
    * Axum is spawned.

So far so good: you have a big web service and a data collector.

The "many routers" are in separate crates & modules. They offer a public interface of:

* An `init` function to setup any internal state.
* A function that returns a `Router` to add to the overall web service.
* A couple of modules expose a `Layer` for use in other modules, mostly API token checking.
* An actor setup similar to what we built that exposes an API for calling that service locally (to avoid making local service calls). They do expose a structure with methods (that call the channel).
* The `tracing` crate is setup with `OpenTelemetry` support to track actual load/response times.

The *focus* is on keeping modules from depending upon other modules, except via channel calls.

It's proven capable of handling thousands of requests per second - many more than they need for now. So no need to scale it up yet! The plan---when and if scaling out is required---is to:

* Find the highest load systems (via telemetry).
* Break them into a separate services that can be run elsewhere. Use protobuf (via `Prost`) and `tonic` (gRPC server), and replace the exposed proxy with gRPC client code. Since Tonic exposes gRPC via a channel - this is a very natural shift.
* We've tested it on some of the simpler modules to be sure it'll work if/when they need it.

## Question 3: How do you divide your work?

* If you like microservices, then you can make lots of tiny services.
* A "modular monolith" works well (see above) for systems you aren't sure if you'll need to scale.
* A giant intertwined monolith hurts!

The most important part is that state is maintained in a manageable way. No "surprise, it changed!" moments.

## Example 1: LibreQoS

LibreQoS is an open source project I contribute to (I'm the CPO of the company that runs it, also). It's a pretty large program that does a few things that are new and innovative. It basically applies traffic shaping at an Internet Service Provider scale, tracks IP data at scale (you can shape and analyze 70 Gbps of data on a $1500 Ryzen box - the most expensive part is the 100 Gbps NIC). We use `Cake` (the scheduling discipline) at scale to fix bufferbloat and apply traffic plans to customers and an HTB hierarchy to model the ISPs network and mitigate congestion - in real time. There's some pretty cool C code in there too, using eBPF to fix Linux's inherently single-threaded traffic shaper. It runs as a transparent bridge in the customers NOC.

It's a good example of a hybrid application with a bit of everything in it.

* `lqos_sys`:
    * Rust wrapper around `libbpf` and `libxdp`.
    * Builds a number of C (slowly being ported to Rust via the Aya project) eBPF programs via `build.rs`.
    * Wraps the programs in an RAII shell that binds them to NICs when started, and clears them when dropped.
    * Provides setup, interface with kernel data structures (eBPF iterators, eBPF's events system) - providing Rust-style interfaces.
* `lqosd`:
    * Has a `tokio::main` - it's mostly async.
    * Validation and setup at start.
    * Holds the `lqos_sys` kernel and ensures it is cleaned up on termination (signals, etc)
    * Launches a "stats ticker" thread:
        * Once per second, eBPF maps are polled giving per-host utilization, TCP round-trip time (the eBPF reads TCP timestamps). It's often collecting tens of thousands of hosts' data - so it's highly tuned.
        * Current state is stored inside this module. We use `DashMap` for lock-free updating.
        * Some state is submitted to channels to other parts of the program.
    * Launches a TCP Retransmission thread:
        * eBPF notifies on every TCP retransmission it detects.
        * These are collated per-host and per-flow via channels to the systems that handle those.
    * Launches a "heimdall" thread:
        * Heimdall is usually idle, but if a user has requested that detailed information about a host be collected then it communicates that intent to the eBPF system and starts receiving packet headers via an eBPF callback system. These are submitted to a channel for storage and can be accessed via the API.
    * Launches a "flow tracker" thread:
        * Periodically polls the "active flows" table and stores the current data in a "hot cache" for browsing.
        * eBPF sends notifications whenever a "flow" (TCP connection, UDP between two hosts with continual port numbers, ICMP etc.) either terminates or has been idle for a time.
        * Terminating flows are removed from the "hot cache" and sent by a channel to a system that runs geolocation on the IP, maps it to ASN, and maintains a "last 5 minutes" buffer of who customers have connected to, the quality of the connection (RTT and latency)
        * In turn, this information is periodically submitted to the "long term stats" system (below).
    * Launches an async task that uses Linux iNotify to watch for configuration changes, and notifies the rest of the program when these occur.
    * Spawns a "control bus":
        * It's a TCP Listener attached to raw TCP on a local UNIX socket.
        * Incoming connections spawn handlers.
        * Local connections are basically a header, size, and the command enum pattern. Tightly serialized with `binpack` (since it never leaves the server, no need to worry about endianness - it'll always be the same).
    * Finally, Long Term Stats receives periodic state updates from just about everything in here.
        * Once per second, stats are accumulated.
        * Once per "collation period", they are condensed to min/max/median for the period and submitted (via raw TCP, compressed and encrypted) to a cloud-based statistics system.

Having the bus architecture makes it easy for a number of other processes to run on-host as well:

* Some Python scripts periodically update data from ISP's CRM systems (mapping customers to policies).
* Other Python scripts call `lqosd` via the bus to build the traffic shaping tree and ensure the mappings between shaper handle and customer/device are present/correct. (We use `PyO3` to offer a full Python bus client, written in Rust).
* `lqos_node_manager` is a local (lightweight) web application that lets you browse current activity in real-time, right down to statistics for individual flows, real-time stats from Cake, and collect packet captures. You can edit your configuration and manage the system. It exclusively gathers data via the bus. This allows it to run as a non-priveleged daemon for security.
* `lqtop` provides a text user interface that offers similar data to `lqos_node_manager`.
* A handful of CLI applications can call the bus and manage it.

Then there's the Long-Term Stats system. We're a small, Open Source project and wanted to keep costs as low as possible. We also wanted to design to scale out if we need to. Currently, our hosting bill is less than $50/month! LTS consists of a few pieces:

* A really big PostrgreSQL + TimeScale database.
* `lts_license_manager`. `lqosd` will check that it's allowed to submit data before it tries, the license manager handles authorization and billing. It also provides us with an (ugly!) admin interface.
* `anonymous_stats`. `lqosd` at various nodes will (unless you didn't opt in) send us anonymous usage statistics. This daemon just stores them.
* `lts_ingestor` receives periodic connections from `lqosd`, double-checks authorization and if authorized processes the submitted data (custom packed format, compressed and encrypted via Dr. Bernstein's sodium system).
    * Valid submissions are submitted to a channel for processing.
    * The channel "fans out" and assesses a whole bunch of different statistics - storing them in TimescaleDB.
* `lts_node_manager` is an Axum-based webserver that lets users view their statistics.
    * `conduit` provides a WASM library for compressing command enumerations and submitting them over WebSockets. It runs in the browser.
    * Minimal static pages provide just enough to get started.
    ` web` is a TypeScript system that builds the entire front-end.

> This architecture has been wonderful to work with. Everything is isolated, and faults don't bring down the whole thing. In 1.5 years, with deployments in 44 countries (and 22 US States) we had: *zero* system crashes.

## Example 2: An Unnamed Fintech Company

I can't name names because of NDAs, but their architecture is a good example.

The system receives financial transaction data from thousands of locations around the world. These are then analyzed programatically, and new data is submitted to the same thousands of locations - and to various analysis systems. The original version ran in Python, and had *huge* hosting costs to keep up.

* At the core, is a cluster of containers all running a Rust system built around Actix.
* Actix is primarily receiving POSTed data from client locations.
    * It utilizes layers for authentication/authorization.
    * Additional layers add customer data and inject dependencies.
    * Submitted data is submitted to a bunch of other services via gRPC or channels (with a proxy facade class so the endpoint developers don't need to worry about the details).
    * It's basically a message-passing core - the ideal case for microservice type architectures.
* The actors (either self-hosted or in-process) are split into two types:
    * Legacy systems still run Python. The messages are received by the Rust service core, and are Python is executed in-process via the `PyO3` library.
    * New systems are written in Rust. (Note: much of the initial analysis is actually still done in Python, they are very much a Pandas shop for design! Polars for Rust is good, but not quite there yet. Once the algorithm is deduced, it is ported to Rust for speed)
* A number of systems submit data to other (pre-existing) systems in the formats they require.

The customer reports very, very few system problems - and that Rust has more than halved their hosting costs.
