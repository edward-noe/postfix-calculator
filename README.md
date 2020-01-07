# postfix-calculator
Command line-based Reverse Polish notation calculator written in Rust

## Usage
To compile, use ```rustc```.
To use, enter integers and operators (+, -, *, /). Enter can be pressed in between inputs without exiting, assuming there are enough operands. To exit the program, press ```Control + D```. The final result will be output.

## Example
```
1 7 4 + 5 - *
[Control + D]
```
will output 
```
[ 6 ]
```
