# Chip-8 Disassembler


A Chip 8 Disassembler implemented in Rust.

## Prerequisites


Building this project requrires both Rust and GNU Make. 
The project is at this time (August, 2018) built with GNU Make V 4.1 and
rustc V 1.28.0. However, the project should build without problems on 
earlier versions of both make and rustc.

## Building


Clone this repository

```
$ git clone https://github.com/AlexOberhofer/Chip-8-Disassembler-Rust.git
```
Switch to working directory

```
$ cd Chip-8-Disassembler-Rust
```

Build

```
$ make
```

The executable will be built in the main project directory. If you wish to 
change the output location feel free to change the output directory the make 
file.

## Usage


Working directory

```
$ ./chip-8-disassembler </path/to/rom/>
```

Other executable location

```
$ </path/to/executable/> </path/to/rom/>
```

## Authors


**Alex Oberhofer**

## License


This project is licensed under the GNU GPL 3.0 License - see the [LICENSE.md](LICENSE.md) file for details
