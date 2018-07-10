# Sandcastle

A scene graph which can be used by a renderer such as sandcastle-vulkano to
generate a display.

## Inspiration

This crate is meant to take successful ideas used by various UI toolkits based
on scene graphs such as Qt's Quick 2, EFL Evas, and GTK+ 4 GSK in a Rust
native library. GSK is especially note worthy as the graph is immutable after
construction which I believe fits well with Rust's memory model to start.
