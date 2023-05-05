#!/bin/bash

if [ "$#" -lt 3 ]; then
    echo "Usage: `basename $0` <output filename> <noise level> color1 color2 ..."
    exit 1
fi

output=$1
noise=$2
shift 2
p=($@)

# Create palette
for ((i=0; i<${#p[@]}; i++)); do
    convert -size 100x100 xc:"${p[$i]}" "/tmp/swatch_$i.png"
done
montage -tile x1 -geometry +0+0 /tmp/swatch_*.png /tmp/palette.png

# Create LUT
convert HALD:8 -duplicate 512 -attenuate $noise +noise Gaussian -quantize LAB +dither -remap /tmp/palette.png -evaluate-sequence Mean $output

# Cleanup
rm /tmp/swatch_*.png
rm /tmp/palette.png

