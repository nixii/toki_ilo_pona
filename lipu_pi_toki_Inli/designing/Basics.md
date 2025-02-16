
# Basics

## Types
* `nanpa`: number (float OR integer, 64-bit)
* `lon`: boolean
* `linja`: string (or char)
* `kulupu<t>`: A list of a type
* `tomo t`: A type for a struct `t`.
* `nasin`: A function
* `o nasin`: A function call object
  * A preset function call with parameters.

## Keywords
> Keep in mind these may be used in many ways.
* `nimi`: Define a variable
* `o`: Call a function. *\<This is also used in a type.>*

## Variables
To define a variable, start with the keyword `nimi`. After that, state the type of the variable. Finally, put the name of the variable. Variables require their names to start with a capital letter. This is because, in Toki Pona, loanwords start with capital letters, and variable names are kind of like loanwords.

After the name of the variable, you put `=` and then its value. There is NO `null` type here.

## Examples

### Making a variable and printing it out
```
nimi linja NimiMi = "soweli Nikusi";
o toki e NimiMi;
```

### Creating a function and calling it
```
nimi nasin Toki = () => {
    o toki e "toki!";
} => ala;

o Toki;
```

### Creating a function with a parameter
```
nimi nasin TokiTawaJanNi = (linja JanNi) => {
    o toki e (o linja.Wan e JanNi e " o, toki!");
} => ala;

o TokiTawaJanNi e "jan ale";
```

### Creating a function with a parameter and a return type
```
nimi nasin PanaENanpa = (nanpa Nanpa) => {
    o pana e Nanpa * 2;
} => nanpa;

o toki e (o nanpa.TawaLinja e (o PanaENanpa e 5));
```