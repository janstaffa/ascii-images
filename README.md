# ASCII-IMAGES

Image to ASCII text convertor. The `/images` folder contains some test images.

#### before:

<img src="assets/before.jpg" width="500" />

#### after:
<img src="assets/after.jpg" width="500"/>

## run:

```bash
# compile
cargo build
# run
./target/debug/ascii-images --in <path to input file> --out <path to output file>

# or

# run directly
cargo run --in <path to file> --out <path to output file>

# example
cargo run --in images/image1.png --out output.txt
```

## characters:

The program uses the "standard" character ramp for grey scale pictures, black -> white i.e.:

```
$@B%8&WM#\*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-\_+~<>i!lI;:,"^`'.
```

## notice:

The output text files are **massive** (there is one character for every second pixel). Some text editors may limit line lengths and distort the result.
