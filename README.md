# Box Crash

This is a simple game in Rust and Piston with a Camera for 3D rendering.
This game was written with hope to be a good example of game development in Rust.

## Demo videos
[![Demo](http://img.youtube.com/vi/iEvYlKGlAPs/0.jpg)](http://www.youtube.com/watch?v=iEvYlKGlAPs "Video Title")

## Build and run

Download binary (64-bit Windows and Linux) here: https://github.com/juchiast/boxcrash/releases

This code should be compiled with the latest stable version of Rust (1.16.0 as of this writing).

Almost all game's constants are configurable via `resources/config.json`.
You should edit the screen size details to match your monitor.

## Gameplay

Drive the box to avoid crashing with others, You can also speed-up, jump, and shoot them.

Control:

- Move left/right: A, D
- Speed-up: W
- Jump: Space
- Stare and shoot: Hold right mouse, then click left mouse

## Known bugs

Game run a bit slow on Windows machine.

Crashed on some Ubuntu machines with a "Cannot find root window" error (I still don't know how to fix).
This seem to be a piston\_window's bug.

## What's next?

- Write an article about the writing of this code.
- Fix some known bugs.
- Test game on more machines.
- ~~Write a GUI to configure and restart game~~ (Done).
- Draw more details of box and the road.
- Add crashing animations.
- Add some sounds.
- Try to build on Web and Android.

## License

MIT
