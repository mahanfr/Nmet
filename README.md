<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/mahanfarzaneh2000/nemet">
    <img src="assets/Nmet.png" alt="Nmet - Nmet programming language" height="100">
  </a>

  <p align="center">
    A general purpose statically typed and compiled programming language 
    <br />
    <a href="https://github.com/mahanfarzaneh2000/nemet#quick-start"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/mahanfarzaneh2000/nemet/issues">Report Bug</a>
    ·
    <a href="https://github.com/mahanfarzaneh2000/nemet/issues">Request Feature</a>
  </p>
</div>

## Installing
For installing in Gnu/Linux system make this projectl.
```shell
$ make
```
or
```shell
$ make install
```

## Quick Start
The Project is in development state and dose not come with a package yet and **It is Only available fot linux** but you can use wsl in windows!
<br />
For getting started clone the repository and build the project using Rust toolchain
<br />
Install [Nasm](https://www.nasm.org/) using your package manager or by downloading it from it's official website
<br />
Create a file like ```hello.nmt``` extention and write a simple program insde it:

lets start with a classic application that prints hello world to the standard output
``` nmt
fun main() {
    print "Hello World!\n";
}
```
now you can run the following commands:

``` shell
$ cargo run ./hello.nmt
$ ./build/output
```
## Compile your code
```
nmt fileName.nmt
```
```nmt fileName.nmt``` generate your project to **build** directory

For example:
```mnt
$ nemet examples/hello_world.nmt
$ ./build/hello_world
```
## About The Project

A General Purpose Compiled Programming Language that generates x86-64 assembly as Intermediate representation (IR) which can be compiled to binary using nasm.
We eventialy will move away from nasm and implement our own loader but the current goal is to become self hosted by writing the compiler in itself!

Use the `docs/README.md` to get started.

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).


## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.


[contributors-shield]: https://img.shields.io/github/contributors/mahanfarzaneh2000/nemet.svg?style=for-the-badge
[contributors-url]: https://github.com/mahanfarzaneh2000/nemet/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/mahanfarzaneh2000/nemet.svg?style=for-the-badge
[forks-url]: https://github.com/mahanfarzaneh2000/nemet/network/members
[stars-shield]: https://img.shields.io/github/stars/mahanfarzaneh2000/nemet.svg?style=for-the-badge
[stars-url]: https://github.com/mahanfarzaneh2000/nemet/stargazers
[issues-shield]: https://img.shields.io/github/issues/mahanfarzaneh2000/nemet.svg?style=for-the-badge
[issues-url]: https://github.com/mahanfarzaneh2000/nemet/issues
[license-shield]: https://img.shields.io/github/license/mahanfarzaneh2000/nemet.svg?style=for-the-badge
[license-url]: https://github.com/mahanfarzaneh2000/nemet/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/mahanfarzaneh
[product-screenshot]: assets/nemet.png
