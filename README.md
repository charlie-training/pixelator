# pixel-thumbnails

Create customisable pixelated thumbnails from images.


## Usage

If running the release executable:

```bash
./pixelator <image_path> <pixel_value>
```

If forking the repo and running in development mode:

```bash
cargo run <image_path> <pixel_value>
```

Where `image_path` is the relative or absolute path to the target image file (including extension), and `pixel_value` is the number of pixels to generate per axis (e.g. a lower value means fewer overall pixels and less fidelity in the output).

## Examples

With a 400 x 400 pixel input and 20 blocks of pixels along the x and y axes:

```bash
./pixelator input.png 20
```

Output:
```bash
Original dimensions: 400px x 400px
Segment size: 20px x 20px
Image saved to output.png, size: 400px x 400px
```
