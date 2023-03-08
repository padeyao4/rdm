# rdm - Recursive Directory MD5

rdm is a command-line tool written in Rust that can calculate the md5 value of a directory recursively. It uses the rust-crypto crate to implement the md5 algorithm and the walkdir crate to traverse the directory tree. It also uses the clap crate to parse command-line arguments and the serde crate to serialize and deserialize data.

## Installation

To install rdm, you need to have Rust 1.56 or higher and Cargo installed on your system. Then you can run the following command:

cargo install rdm
This will download and compile rdm and place it in your Cargo bin directory.

## Usage

To use rdm, you need to provide a directory path as an argument. For example:

rdm C:\Users\example\Documents
This will calculate the md5 value of the Documents directory and print it to the standard output in a human-readable format. You can also use relative paths or multiple paths as arguments.

By default, rdm will ignore subdirectories and hidden files when calculating the md5 value. You can change this behavior by using some optional flags:

-r or --recursive: Include subdirectories in the calculation.
-a or --all: Include hidden files in the calculation.
-s or --sort: Sort files by name before calculating the md5 value.
-j or --json: Output the md5 information in JSON format.
-h or --help: Print a help message and exit.
-v or --version: Print version information and exit.
For example:

rdm -r -a -j C:\Users\example\Documents
This will calculate the md5 value of the Documents directory recursively, including hidden files, and output it in JSON format.

## Output

The output of rdm depends on whether you use the -j or --json flag. If you do not use this flag, rdm will output a simple text message that contains two parts: The directory path that was given as an argument and its corresponding md5 value. For example:

C:\Users\example\Documents: d41d8cd98f00b204e9800998ecf8427e
If multiple paths are given as arguments, rdm will output one line for each path. For example:

C:\Users\example\Documents: d41d8cd98f00b204e9800998ecf8427e
C:\Users\example\Downloads: c4ca4238a0b923820dcc509a6f75849b
If you use the -j or --json flag, rdm will output a JSON object that contains two fields: path and md5. The path field is a string that represents the directory path that was given as an argument. The md5 field is a string that represents the hex digest of the md5 value of that directory. For example:

{
"path": "C:\\Users\\example\\Documents",
"md5": "d41d8cd98f00b204e9800998ecf8427e"
}
If multiple paths are given as arguments, rdm will output an array of JSON objects, one for each path. For example:

[
{
"path": "C:\\Users\\example\\Documents",
"md5": "d41d8cd98f00b204e9800998ecf8427e"
},
{
"path": "C:\\Users\\example\\Downloads",
"md5": "c4ca4238a0b923820dcc509a6f75849b"
}
]

## License

rdm is licensed under the MIT license. See LICENSE for more details.
