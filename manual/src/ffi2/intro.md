# C++ Interop

We dealt heavily with C interop. This is *the* most common form of interop, and a lot of places will create a C-friendly "wrapper" around their C++ code and just use that. It's not ideal if you've built modern C++ everything and rely on lots of classes - but it works.

Another option is to use DTolnay (the author Serde)'s [CXX.rs](https://cxx.rs/) system. It's a bit picky about the C++ it works with, but it gives you a head start on combining the two languages.

The CXX library is quite new, but is being used by Meta.