# Actor Concept

The actor model was the original design behind Object Oriented Programming, and lives on in Objective-C and Smalltalk derivatives. In a distributed sense, it drives Erlang - known for its reliability.

The idea behind an "actor" is that a service is isolated and retains its own state. It *only* communicates with other services via message passing (whether via a channel or the network).

This means:

* (Upside) Actors are well suited to larger teams who need to operate in relative isolation from one another.
* (Upside) A well-architected actor model can be fault tolerant in a distributed network.
* (Downside) Message passing isn't free, and there's a (small for channels, less small for network) price to pay for passing messages back and forth.

Typically, you'll isolate your actor in its own module and keep all of the state storage private (whether its a database or in-memory). You'd still use synchronization and `Arc`, but all of the details are hidden from consumers.

It's common to use Actix, but you can pretty easily implement an actor system with Tokio.

There are three common models. Most larger projects wind up mixing them:

* An "on demand" model. Scales well, needs care to avoid "spin up" cost.
    * You expose a function that "spawns" the actor's main function and returns either a proxy class or a bare channel.
    * Each spawned actor acts independently - state is shared inside the module.
* A "single actor" model. For actors that are commonly used but don't need to scale out.
    * You expose a function that spawns an actor at application start-up.
    * You keep the channel sender stored, and a connection request just clones another sender and returns it.
    * This is the simplest for state management, because the actor can hold all of its state as part of the initial startup.
    * Can potentially be overwhelmed by huge numbers of requests that take time to process. To mitigate:
        * Requests should be executable in their own async future (via spawn).
        * Requests that need a reply should move the reply into separate handling.
        * This allows the benefit of multiple actors with only one implementation.
* An "actor pool" model.
    * Same thing, but you spawn several and load-balance between instances.
    * State management once again requires synchronization.
    * This is the least common model in my experience.

There are also two common communication patterns:

* An `enum` enumerating all of the commands, with a channel that handles these.
* A `proxy` class that hides the channel and provides a nice API from the outside. The "proxy" is typically published as part of the public part of the module.
