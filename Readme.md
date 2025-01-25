# Nana

Nana is a simple, dynamically typed, interpreted language. It is designed to be
easy to learn and use. The Nana interpreter runs as a WebAssembly component.

To get a feel for what Nana programs look like here is an example Fizzbuzz
implementation:

``` haskell
# A mostly functional fizzbuzz example
Package nana:examples@0.0.1

World fizzbuzz [
   Import wasi:cli/stdout
   Import wasi:streams/output-stream

   Export print-fizzbuzz [max u8] _
]

Func num-to-txt [num u8] string
  Match [mod(num 3) mod(num 5)] [
    [0 0] "Fizzbuzz"
    [0 _] "Fizz"
    [_ 0] "Buzz"
    [_ _] n
  ]

Func list-to-txt [list list<u8>] list<string>
  map(num-to-text list)

Func print-fizzbuzz [max u8] _
  Let[lines  list-to-text(range(1 100))
      stdout stdout/get-stdout()]
    For[line lines]
      stdout.write(line)
```

If you'd like to learn more please read [the language
tutorial](./blob/main/examples/learn_x_in_y_minutes.nana) written in the [Learn
X in Y minutes](https://learnxinyminutes.com) style. For a more formal look
at the language there are [Railroad diagrams describing Nana's
grammar](./blob/main/docs/grammar/index.md)