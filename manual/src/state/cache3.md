# You've Made Memcached V1

The original `memcached` really was this simple, and it's pretty easy to create. Notice that you've wrapped the whole thing in an `Arc`---so you can clone it and make it available *anywhere* in your program. It's small, efficient, and self-managing. For acting as a layer between frequent lookups and a data source, it could be very useful.

