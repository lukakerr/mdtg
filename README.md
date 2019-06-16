# Markdown Table Generator `mdtg`

[![Crates.io](https://img.shields.io/crates/d/mdtg.svg)](https://crates.io/crates/mdtg)

Quickly generate markdown tables in your terminal using a simplistic specification.

### Installation

#### `crates.io`

```bash
$ cargo install mdtg
```

#### Manual

```bash
$ git clone https://github.com/lukakerr/mdtg.git
$ cargo run
```

### Usage

```bash
# Create a 4 by 4 table, without spaces in argument
$ mdtg 4x4

|        |        |        |        |
| ------ | ------ | ------ | ------ |
|        |        |        |        |
|        |        |        |        |
|        |        |        |        |

# Create a 3 by 5 table, with left, center and right aligned columns
$ mdtg "3lcr x 5"

|        |        |        |
| ------ |:------:| ------:|
|        |        |        |
|        |        |        |
|        |        |        |
|        |        |        |
```

### BNF Grammar

```
Spec      -> Column Cross Row
Column    -> Num Positions
Row       -> Num
Num       -> Digit | Digit Num
Digit     -> [0-9]
Cross     -> "x"
Positions -> Position | Position Positions
Position  -> "l" | "r" | "c"
```
