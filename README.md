# elfreader
A simple tool for reading ELF headers written in Rust

# The tool is feature incomplete in the moment

This tool aims to provide information stored in ELF files like executables and shared libraries. 
These information include the identification information and the section headers

## Execute
just type in the following command

```cargo run <name-of-the-elf-file>```

You can also first build the elfreader with

```cargo build```

and then move the built executable into your path. Then you can simply use it via

```elfreader <name-of-the-elf-file>```

## List of identified architectures
* AT&T WE 32100
* SPARC
* x86
* x86-64
* Motorola 68000 and 88000
* Intel MCU
* Intel 80860
* Intel 80960
* MIPS and MIPS RS3000 little endian
* HP PA-RISC
* PowerPC(32 and 64 bit)
* S390 and S390x
* ARM(64 bit included)
* SuperH
* IA-64
* TMS 320C6000 Family
* RISC-V
* BPF
* WDC 65C816

## List of identified ABIs
* SysV
* HP-UX
* NetBSD
* Linux
* GNU Hurd
* Solaris
* AIX
* IRIX
* FreeBSD
* Tru64
* Novell Modesto
* OpenBSD
* OpenVMS
* NonStopKernel
* Aros
* FenixOS
* CloudABI
* OpenVOS

The hex values for the architectures are taken fron [Wikipedia](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
