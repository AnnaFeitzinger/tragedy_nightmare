# gene parsing/extraction

## usage
1. install cargo and rust with [`rustup`](https://rustup.rs/).
2. clone the repository with `git clone https://github.com/AnnaFeitzinger/tragedy_nightmare.git && cd tragedy_nightmare`
3. edit `src/main.rs` to your specification. (mostly just change the genome file for each one you would like to filter)
4. optional: if you are using flybase (recommended), download your input genomes/original `.fasta` files from <https://ftp.flybase.net/genomes/>, choosing each species you want to filter's file, and choosing the latest release from 2019, 2018, 2017, or if needed the current genome release. we recommend the `/genomes/species/fasta/<genome>-all-r<version>-gene.fasta` file, as it includes all genes indexed in the genome. move these files into the [`/data`](./data) directory.
5. download/move your original `.fasta` files into the data directory, and modify the `src/main.rs` accordingly (you will have to `cargo run` multiple times for multiple file, changing `main.rs`, specifically the `extract_sequence` function input as the new path, with the new files each time, along with the `file_writer`'s output file, for each input file change)
6. build the portable binary with `cargo build --release`. (this comes with the hard-coded input and output names, so it's not recommended)
7. run the binary on the system with `cargo run --release`. (this will run the command instantly on your platform, and changes if you change input and output and re-run)
8. to target the binary to different platforms, see [rust's documentation on cross compilation](https://rust-lang.github.io/rustup/cross-compilation.html)

## information/specifications
- expects an ortholog file of a certain type, see the `data` directory for file structures and types.
- expects that all "ids" are unique to their individual genome/species. (true for flybase gene ids)
- expects the fasta file to have a parsable name header on each gene sequence. (true for most fasta-spec files)

## license (or lack thereof)
this project is under [The Unlicense](https://unlicense.org), as defined in the [`LICENSE` file](./LICENSE) and below:

```text
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org>
```
