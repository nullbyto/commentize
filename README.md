
# Commentize

This is a simple command-line tool to turn any text or file into a nice looking comment box with options to change the box symbols and box dimensions.
The "commentized" text can be outputted to the terminal or added to one or more source files **Recursively**, with the option to add the text to the top or the bottom of the source file/s. The tool won't duplicate the *commentized* text if it's already added.

<!-- Example -->
### Example:
![](https://github.com/nullbyto/commentize/raw/master/examples/1.png)

In this example we *commentized* a string "Hello Github" with right padding length 10 (-r), height padding length 2 (-y) (top/bottom) and edited the wall symbols to "|||" (-w). Which resulted in a nice looking comment box as shown.

Click [here](https://github.com/nullbyto/commentize/tree/master/examples) to see all examples.
  

<!-- GETTING STARTED -->
## Getting Started

This is an example on setting up the tool locally.
To get a local copy up and running follow these simple steps.

 There are two ways to get the tool:
 - Build it from source (recommended)
 - Download it pre-compiled from the [releases](https://github.com/nullbyto/commentize/releases) page

### Prerequisites

If you choose to build the tool from source you need to have `rustc` and `cargo` installed:
* Follow the instructions on https://rustup.rs/ to download and install rustc alongside cargo.

### Building from source

1. Clone the repo and change to the directory

```sh
$ git clone https://github.com/nullbyto/commentize.git
$ cd commentize
```

2. Build the project
```sh
$ cargo build --release
```

3. Find your binary in the directory: **target/release**

**Extra**: to be able to use the tool from anywhere on your pc, you need to add the directory with the binary into `PATH`.


<p  align="right">(<a  href="#top">back to top</a>)</p>

<!-- USAGE -->
## Usage and options

Usage and options should be self-explanatory from the **--help** menu.

**Note** you have to be located in the directory of the binary file to be able to use it unless it is in the `PATH`, which will not be covered how to do here.

```sh
Commentize 0.1.0

USAGE:
	commentize.exe [FLAGS] [OPTIONS] --comment <comment> --file <file> [path]

FLAGS:
	-a, --append 	Append to file path
	-h, --help 		Prints help information
	-m, --modded 	Modded style
	-o, --output 	Output the comment
	-V, --version 	Prints version information

OPTIONS:
	-c, --comment <comment> 	Comment to be added
	-f, --file <file> 			Text file to get comment from
	-y, --hp <height-pad> 		Height padding length
	-l, --left <left> 			Left side padding length
	-r, --right <right> 		Right side padding length
	-s, --symbol <symbol> 		Symbol to commnetize with
	-w, --wall <wall> 			Wall symbol to commnetize with
	-x, --wp <width-pad> 		Width Padding length

ARGS:
	<path> 		Path to file or directory to commentize in the header
```

  <p  align="right">(<a  href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

Don't forget to give the project a star!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m "Add some AmazingFeature"`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p  align="right">(<a  href="#top">back to top</a>)</p>
