# 42 scop

A program that renders 3d objects (from Wavefront .obj files), written in Rust using OpenGL API.

## Requirements

### Rust compiler + cargo 

https://www.rust-lang.org/tools/install

MacOs and Linux:

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### GLFW 

https://www.glfw.org/

MacOs:

```console
brew install glfw
```

## How to run

```console
make
./scop {path/to/obj}
```
## Keybindings

- W / A / S / D / R / F : move the camera around
- SPACEBAR : smoothly appy / remove texture
- ESC : exit
- LEFT-MOUSE + DRAG : rotate the model
- RIGHT-MOUSE + DRAG : rotate the camera
- T / G : toggle wireframe mode on / off
