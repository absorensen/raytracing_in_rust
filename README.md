# raytracing_in_rust
A path tracer based on [Peter Shirleys books](https://raytracing.github.io/). I tried to take the end of the third book as a jumping off point for learning more about Rust and various quality of life improvements and performance enhancements. I added a UI with [egui](https://github.com/emilk/egui) and increased performance with [ultraviolet](https://github.com/fu5ha/ultraviolet) and [rayon](https://github.com/rayon-rs/rayon). I removed all of the smart pointers, dynamic dispatches and recursive rendering, replacing them with dependency injection of services and using indices, enums for handling structs implementing an interface and recursion was replaced by a loop based on [pbrt's render loop](https://pbr-book.org/3ed-2018/Light_Transport_I_Surface_Reflection/Path_Tracing#fragment-Intersectmonoraywithsceneandstoreintersectioninmonoisect-0).  Ultimately, as I wanted to implement more advanced topics, the overly paedagogical architecture from Peter Shirleys books proved to be hard to continue to adapt without massive changes. 

It was a great experience and can wholeheartedly be recommended. Check the images folder for results representative of the current state of the path tracer.
