# elfreader

A simple tool for reading ELF headers written in Rust. This is my way to learn the ELF file format.
Inspired by the GNU binutil `readelf`

# The tool is feature incomplete in the moment

This tool aims to provide information stored in ELF files like executables and shared libraries. 
These information include the identification information and the section headers

## Implemented features:
- [x] ELF header detection
- [x] Generic program header detection
- Detection of OS specific program header data
- [x] Generic section header detection
- Detection of OS specific section header data
- [x] Proper basic CLI interface
- Other features I have not thought of so far

## Build

Download the source code and execute

```cargo build --release```

in the directory with the `README.md` to get a release build. This requires you having installed the rust toolchain on your system.
Works with rust 1.61 and later. Previous versions are untested.

After building the binary is under `<path-to-code>/target/release/elfreader`
## Execute
just type in the following command

```cargo run <name-of-the-elf-file>```

You can also first build the elfreader with

```cargo build```

and then move the built executable into your path. Then you can simply use it via

```elfreader <name-of-the-elf-file>```

For more information about this tool use

```elfreader --help```

The hex values of architectures, word size etc are looked up at [Wikipedia](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)