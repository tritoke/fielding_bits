# fielding bits
A rust program for generating images like those found in this [twitter thread](https://twitter.com/aemkei/status/1378106731386040322).
It's technically multithreaded but its not necessary, I only did it because [rayon](https://docs.rs/rayon/1.5.0/rayon/) is so easy to use.

## Getting started
``cargo run --release``

Yep its that simple :)

If you want to change the renders you need to edit the source itself, specifically these lines:
```rust
let fr = |x, y| (x * y) & 64 != 0;
let fg = |x, y| (x | y) % 17 == 0;
let fb = |x, y| (x ^ y) & 32 == 0;
```

Respectively these functions define whether to enable the red/green/blue channels for each pixel, based on their position in the image.

## Scaling
I would recommend this command for upscaling images: ``convert render.png -filter point -resize 400% upscaled.png``

## Animation
There is a `video` branch which contains code for generating video frames out to a directory.
These can then be stitched together using the following `ffmpeg` command:
```
ffmpeg -f image2 -r 16 -i frames/%05d.png -c:v h264_nvenc -qp 0 video.mp4
```

## Examples
![A checkerboard pattern, which has a checkerboard nested in every other square. Over all this there is a pattern of concentric circles.](https://github.com/tritoke/fielding_bits/blob/main/examples/checkerboard_mad.png)

![A grid where each tile contains a load of concentric circles.](https://github.com/tritoke/fielding_bits/blob/main/examples/funky.png)

![A gif where the screen slowly fills with dots.](https://github.com/tritoke/fielding_bits/blob/video/examples/dots.gif)
