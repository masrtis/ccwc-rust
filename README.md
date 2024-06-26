# Coding Challenges - wc

This repository implements the [wc Coding Challenge](https://codingchallenges.substack.com/p/coding-challenge-1) in Rust.

## Build instructions

This program was tested on Windows 11 and a combination of the Rust and Python Docker images.

Dependencies
- [Rust](https://www.rust-lang.org/learn/get-started).
- [Python](https://www.python.org/about/gettingstarted/)
- [Pytest](https://docs.pytest.org/en/8.2.x/getting-started.html)
- [Docker](https://www.docker.com/get-started/)

To build the Rust binary:
- `cargo build`

To run the Rust binary:
- `cargo run`

Other cargo options can be used as needed.

The provided Dockerfile includes steps to build the Rust binary using cargo and runs the Python tests. This avoids needing to install dependencies manually.

To run the Docker images specified in the Dockerfile:
- `docker build -t ccwc-rust .`
- `docker run -it ccwc-rust`

## Implementation and Test

The main.rs file is responsible for parsing, preparing, and processing commands that are specified as arguments to the program. The valid commands that can be specified are:
 - `-w` - counts words in text file
 - `-c` - counts bytes in file
 - `-m` - counts multibyte characters in text file
 - `-l` - counts lines in text file
 
Only one command can be specified. One optional file path can be specified, which will open the file to execute the command specified. If no file is specified, the command specified will be executed against input provided from the standard input stream. If no commands are specified, the commands `-l`, `-w`, and `-c` will be run.

Integrations tests written in Python are provided. The tests will launch the executable specified by the environment variable `CCWC_PATH` or will attempt to run the binary at `./target/release/ccwc-rust.exe`. The test data path that is passed on the command line when running the tests is either specified at the environment variable `TEST_DATA_PATH` or attempted to be located at `./integration_tests/text.txt`. Tests are included to test each command against the test data provided as part of the challenge, testing input from stdin with a command specified, testing only passing a valid file path, and passing a nonexistant file path as the only argument.

A Dockerfile is provided. The image has a build and a test stage. The build stage is based off the latest Rust image that copies the source code and uses cargo to build the executable. The test stage is based off the latest Python image. The test stage installs pytest, copies the contents of the integration_tests directory into the container and the built executable from the build stage, sets the aforementioned environment variables, then runs pytest to execute the tests.

## Future work

The application is not 100% compatible with wc - it does not support passing in more than 1 command and a file name, nor does it support passing in multiple files.

Rust based unit tests would be nice to implement, as would splitting the Rust code into separate modules.
