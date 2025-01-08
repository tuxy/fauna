![Showcase demonstrating a number of falling balls with customisation with rapier physics](./showcase.gif)

# 3D Physics showcase
This is a new attempt at trying to implement physics into a "draw"-based graphics library. Previously, I tried creating sphere -> sphere & plane physics with [raylib](https://www.raylib.com/) and my own physics. Now, I'm using the rust-based [macroquad](https://macroquad.rs) and [rapier](https://rapier.rs/) physics.

While this project is a continuation of my last one, the goals are a little different. The last one focuses on the actual physics, and this newer one focuses on the simulation part, which can be easily modified and customised WITHOUT looking at the code

# Features
- Full sphere -> plane and combinations between them
- (Partial) cuboid collision support (No angular momentum or rotation)
- Sphere -> Plane physics can be modified in the megaui window
    - Also includes world options such as gravity, friction and restitution
- High performance (Though this is more due to the better implemented and performant physics engine)