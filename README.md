# pokercalc

This is a very simple tool I made to help me play through scenarios in Texas
Hold 'em no limit poker to better understand the odds I faced.

It determines the hole cards an opponent needs to beat you given the cards
you can see, and the probability they were dealt to your opponent at the
start of the hand.

## Building and running

This is written in [Rust](https://www.rust-lang.org/). To build the program,
install the Rust toolchain with Cargo, clone the code, and fire off
a `cargo run` command to compile and run the software.

## Usage

Either type in, or feed into stdin, your hole cards followed by whatever's been
dealt so far on the board - the flop, turn, and river – all space separated.
Cards should be of the format `XY` where `X` is the value
(`2` to `9`, `T`, `J`, `Q`, `K`, `A` for two to nine then ten, jack, queen,
king and ace) and `Y` is the suit (`H`, `D`, `S`, `C` for hearts, diamonds,
spades and clubs). The software is case insensitive.

For example, entering...

`jd 9d 7s ac 9h`

...describes the jack of diamonds and nine of diamonds as your hole cards,
then the seven of spades, ace of clubs, and nine of hearts on the flop.

After entering the cards, hitting enter, or sending a newline, it will list
combinations of hole cards your opponent needs to beat you, and the probability
they were dealt one of those combinations.

Hitting Control-C, Control-D, sending an EOF or closing stdin will end the
program.

## Development, license, and contact

If you want to improve this software, shoot me a pull request. There are a few
more features I want to add to it, so consider it a work in progress. I also
need to write some comprehensive tests to make sure I've caught all the corner
cases.

The code is licensed under the standard MIT License – see the LICENSE file
for the terms and conditions of use. If you want to get hold of me
[email me](mailto:diodesign@gmail.com),
or message me [on Twitter](https://twitter.com/diodesign).
