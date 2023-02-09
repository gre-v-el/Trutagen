# Truth Table Generator
## An easy to use logic expression parser and evaluator
<br>

---
<br>

Example input:
```
not a xor b and (c or a)
```
Example output:
```
┏━━━━━┳━━━━━┳━━━━━┳━━━━━━━━━┓
┃  A  ┃  B  ┃  C  ┃ result  ┃
┣━━━━━╋━━━━━╋━━━━━╋━━━━━━━━━┫
┃  0  ┃  0  ┃  0  ┃    1    ┃
┃  0  ┃  0  ┃  1  ┃    1    ┃
┃  0  ┃  1  ┃  0  ┃    1    ┃
┃  0  ┃  1  ┃  1  ┃    0    ┃
┃  1  ┃  0  ┃  0  ┃    0    ┃
┃  1  ┃  0  ┃  1  ┃    0    ┃
┃  1  ┃  1  ┃  0  ┃    1    ┃
┃  1  ┃  1  ┃  1  ┃    1    ┃
┗━━━━━┻━━━━━┻━━━━━┻━━━━━━━━━┛
 NOT A XOR B AND ( C OR A )
 ```

## Supported operations:
* Or
* Nor
* Xor
* And
* Nand
* Not

## Precedence
`Or`, `Nor` and `Xor` are all treated equally and are evaluated left to right. `And` and `Nand` are also equal, however, with higher precedence than the or-family. `Not` has the highest precedence. Expressions in parentheses are evaluated before the rest is.