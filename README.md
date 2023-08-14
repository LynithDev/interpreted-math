# Weird Interpreted Math Language
Wanted to practice Rust, so I made an interpreted language specifically designed to do math.

It's not optimised at all, and the code is probably terrible, but it works!

## Eg script:

```php
# This is a comment
# whitespaces are ignored so even $var=3+2_0 should work
# Numbers can be seperated with _ for readability (eg 1_000_000 is 1000000)

$var = 3+ 2_0 # This is a variable

# The return value in the script is the first statement which isn't a variable / function assignment
23 # The output of the script is 23 despite assignments above
```

```php
$a = 3
$b = 4 + 1
$c = a + b # To use variables, you don't use the $ symbol, letters are syntax / variables
c
```