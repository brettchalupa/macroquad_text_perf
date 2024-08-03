# Macroquad Text Perf Text

Testing out drawing text on the screen with [Macroquad 0.4.11](https://crates.io/crates/macroquad).

Clone and run with: `cargo run --release`

## Results

I'm on an Apple M2 Pro running on a MacBook Pro.

Default `draw_text`, it takes about 3500 texts drawn to screen to start
dropping frames with `cargo run --release`, **3,000** runs at a steady 60 FPS
for me.

In debug, it’s around ~50 to start seeing frames drop a little.

Compared to Tetra 0.8.0: https://github.com/brettchalupa/tetra_text_perf, which sees
major frame drops in release at even 50 pieces of text.
