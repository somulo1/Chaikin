# Chaikin Curve

A simple Rust project to visualize and animate Chaikin curves. This project uses a graphical window to allow users to interactively add control points, clear the canvas, and animate the refinement of Chaikin curves.

## Features

- **Interactive Control Points**: Click to add control points that form the base curve.
- **Animation**: Animate the Chaikin curve generation process.
- **Canvas Controls**:
  - **Left Mouse Click**: Add a control point.
  - **C Key**: Clear all control points.
  - **Space Key**: Start/Stop the animation.
  - **ESC Key**: Exit the application.
- **Helpful On-Screen Text**: Instructions are drawn on the canvas.

## Dependencies

- [minifb](https://crates.io/crates/minifb) – For creating the window and handling input.
- [nalgebra](https://crates.io/crates/nalgebra) – For vector math and point operations.
- Rust (stable version, edition 2021 is recommended)

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/pochieng/chaikin.git
cd chaikin
```

BUild the project
``` 
cargo build -- release

```

run the application 
```
cargo run ---release

```

```md
# Chaikin Curve

A simple Rust project to visualize and animate Chaikin curves. This project uses a graphical window to allow users to interactively add control points, clear the canvas, and animate the refinement of Chaikin curves.

## Features

- **Interactive Control Points**: Click to add control points that form the base curve.
- **Animation**: Animate the Chaikin curve generation process.
- **Canvas Controls**:
  - **Left Mouse Click**: Add a control point.
  - **C Key**: Clear all control points.
  - **Space Key**: Start/Stop the animation.
  - **ESC Key**: Exit the application.
- **Helpful On-Screen Text**: Instructions are drawn on the canvas.

## Dependencies

- [minifb](https://crates.io/crates/minifb) – For creating the window and handling input.
- [nalgebra](https://crates.io/crates/nalgebra) – For vector math and point operations.
- Rust (stable version, edition 2021 is recommended)

```

## contributers
Philip Ouma #pochineg

James Muchiri #jmuchiri

Samuel Omulo  #somulo