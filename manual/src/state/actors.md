# Actors (best in async)

Using the various actor frameworks (Actix provides one, Tokio provides a great foundation for one) can be ideal for managing state in message-oriented architectures (whether direct actor messages or receipt of message queue data). An "actor service" becomes responsible for the maintenance of a piece of state, and serves both that state and messaging updating the state to other portions of the program.

Actors are also a good proxy for microservices as you scale out (the actor becomes remote).

I mentioned async specifically because async reduces the largest overhead of the actor model: message passing. Typically, in-process actor communications occur over channels. When a channel is crossing thread boundaries, you are accepting a small (16 us on Linux by default) latency for the receiver thread to be activated (unless it has a sufficient stream of messages to remain active). An async system can often "wake up" a polling message task faster than Linux will "wake up" a dormant receiver thread.