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

## Usage
### Without arguments
The program will start and ask you for the expression
```
D:\[...]> trutagen
Enter a logic expression
a or b xor c


┏━━━━━┳━━━━━┳━━━━━┳━━━━━━━━━┓
┃  A  ┃  B  ┃  C  ┃ result  ┃
┣━━━━━╋━━━━━╋━━━━━╋━━━━━━━━━┫
┃  0  ┃  0  ┃  0  ┃    0    ┃
┃  0  ┃  0  ┃  1  ┃    1    ┃
┃  0  ┃  1  ┃  0  ┃    1    ┃
┃  0  ┃  1  ┃  1  ┃    0    ┃
┃  1  ┃  0  ┃  0  ┃    1    ┃
┃  1  ┃  0  ┃  1  ┃    0    ┃
┃  1  ┃  1  ┃  0  ┃    1    ┃
┃  1  ┃  1  ┃  1  ┃    0    ┃
┗━━━━━┻━━━━━┻━━━━━┻━━━━━━━━━┛
        A OR B XOR C
```
### With an argument
The program will use the argument as an input. Note: you can input only one argument. In other cases the program will ask you for manual input. 
```
D:\[...]> trutagen "a or b xor c"


┏━━━━━┳━━━━━┳━━━━━┳━━━━━━━━━┓
┃  A  ┃  B  ┃  C  ┃ result  ┃
┣━━━━━╋━━━━━╋━━━━━╋━━━━━━━━━┫
┃  0  ┃  0  ┃  0  ┃    0    ┃
┃  0  ┃  0  ┃  1  ┃    1    ┃
┃  0  ┃  1  ┃  0  ┃    1    ┃
┃  0  ┃  1  ┃  1  ┃    0    ┃
┃  1  ┃  0  ┃  0  ┃    1    ┃
┃  1  ┃  0  ┃  1  ┃    0    ┃
┃  1  ┃  1  ┃  0  ┃    1    ┃
┃  1  ┃  1  ┃  1  ┃    0    ┃
┗━━━━━┻━━━━━┻━━━━━┻━━━━━━━━━┛
        A OR B XOR C


(base) PS D:\pliki\programowanie\rust\projs\commandline\truth_table_generator\target\release>
```

## The help command
Calling `trutagen help` results in this output:
```
D:\[...]> trutagen help


┏━━━━━━━┳━━━━━━━━━┓
┃ HELP  ┃ result  ┃
┣━━━━━━━╋━━━━━━━━━┫
┃   0   ┃    0    ┃
┃   1   ┃    1    ┃
┗━━━━━━━┻━━━━━━━━━┛
       HELP
```
~~I hope this makes sense.~~