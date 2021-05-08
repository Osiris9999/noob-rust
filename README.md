# noob-rust

## Installation

### For Linux or MacOs

``` curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh ```

### For Windows

Follow the instructions on https://www.rust-lang.org/tools/install to install rustup on windows

Do not forget to install C++ build tools for Visual Studio 2013 or later from the link above

### Post Installation

After Installing rustup on any Operating system, run the following commands:

``` rustup update ```

To check if rust is successfully installed, run  ``` rustc --version ```

## Running the Code

To compile the code simply change directory to the src/main.rc under the respective project and run: 

``` rustc main.rs ```

To finally run the code do ./main.rs on Linux/MacOs or .\main.rs on Windows
