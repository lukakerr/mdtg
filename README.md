# Markdown Table Generator `mdtg`

Quickly generate markdown tables in your terminal using a simplistic specification.

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