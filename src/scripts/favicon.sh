#!/usr/bin/env sh
#
# Generate favicon assets from source
#

magick src/assets/barrel.png -filter Lanczos2 -resize 32x32   -unsharp 0x0.75+1.0+0 -quality 85 favicon-32.avif
magick src/assets/barrel.png -filter Lanczos2 -resize 256x256 -unsharp 0x0.75+1.0+0 -quality 85 favicon-256.avif
magick src/assets/barrel.png -filter Lanczos2 -resize 180x180 -unsharp 0x0.75+1.0+0             apple-touch-icon.png
