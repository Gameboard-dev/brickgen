# BrickGen

A Rust-based procedural generation tool for creating maze-like structures and exporting them as Brickadia save files. This uses various algorithms for maze generation, image processing, and geometric decomposition.

![image](https://github.com/user-attachments/assets/ba0718da-c417-41ea-a937-eed51f4e1404)

## Features

- **Maze Generation**: Generate circular mazes with customizable parameters such as ring count, ring gap, and initial divisions.
- **Bitmap Rendering**: Render mazes as bitmap images with arcs and lines representing walls.
- **Geometric Decomposition**: Decompose shapes into rectangles and triangles for representation.
- **Brickadia Export**: Export generated structures as `.brs` files compatible with Brickadia.
- **Customizable Parameters**: wall width, wall height, granularity, and seed values for unique outputs.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/brickadia-maze-generator.git
   cd brickadia-maze-generator
1. **Install Dependencies: Ensure you have Rust installed. Then, run:**
   ```bash
   cargo run

## Dependencies
This project uses the following Rust crates:
- **brickadia:** For Brickadia save file manipulation.
- **strum and strum_macros:** For enum iteration and display.
- **image and imageproc:** For image processing and rendering.
- **rayon:** For parallel computation.
- **num-traits:** For numeric conversions.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.

## Acknowledgments
Inspired by procedural generation techniques and the Brickadia community.
Special thanks to the authors of the dependencies used in this project.

