# Budget CLI

> **Welcome from Budget CLI, your go-to command-line interface for personal finance management**

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

## Install

Add `bud` to path.

```Bash
|> cargo install --path .
```

## Examples

To `view` budget or transactions, summery or goals:

* view summery report, you can filter by `--id`, `--name`, `--tag` or `--lifetime`, defaults to `1y`.

    ```Bash
    |> bud view summery
    ```

* view goals, you can filter by `--tags`.

    ```Bash
    |> bud view goals 
    ```

To `set` Financial Goals by `--tag`, `--name` and `--amount`

```Bash
|> bud set travel vacation 1000
```

To `delete` a single transaction use the command `bud delete`:

```Bash
|> bud delete --id
```

To `edit` a single transaction use the command `bud edit`:

```Bash
|> bud edit --id Netflix 12€ 100320 1m12x .movies .covid
```

To `add` a single transaction use the command `bud add` by `--name*`, `--amount*`, `--start_date`, `--lifetime`, `--tag` and `--type`

* **Start date** defaults to `current date`
* **Lifetime**: defaults to `1d`
* **Type** defaults to `expense` but you can set it as `income`:

```Bash
|> bud add Salary 1300€ 1m --type
|> bud add Netflix 7.99€ 100320 1m12x .subscription
```

A transaction as the properties:

* **Name**: a descriptive name
* **Amount**: the amount in monetary currency
* **Start date**: the start date since when the lifetime should be computed
* **Lifetime**: a duration that the transaction applies to
* **Tags**: for grouping transactions
* **Type**: a transaction type `expense` or `income`, defaults to expense

### Parsing rules

The library tokenize the input string and looks for the patterns listed below. Anything that cannot be recognized as a pattern it will set as the title of the transaction. The title is **required**

#### Amount

The monetary value of the transaction, **required**:

```EBNF
Amount ::= Natural ( '.' Digit Digit? )? "€" 

Natural ::= NaturalDigit Digit*
NaturalDigit ::= #'[1-9]'
Digit ::= "0" | NaturalDigit 
```

Currently the only available currency is `€`

Examples:

* `10€`
* `10000.99€`

#### Lifetime

The duration of transaction, optional, defaults to `1d`.

```EBNF
Lifetime ::= Duration Repeat?

Duration ::= Natural TimeUnit
Repeat ::= Natural "x"
TimeUnit ::= "d" | "w" | "m" | "y"
```

where the `TimeUnit` is:

* `d` days
* `w` weeks
* `m` months
* `y` years

Examples:

* `1m12x` => one month for 12 times, for example for monthly expenses like monthly subscriptions (Netflix, etc)
* `12m` => twelve months for 1 time, same as `1y`
* `1w52x` => one week 52 times, for example weekly groceries expenses for all the year

> 💡 the number of repeats they influence the total amount of the transaction: `10€ 1m12x` will result of a transaction of total amount of `120€` while `12m1x` will result in a single transaction of `10€` over 12 months

#### Start date

The transaction start date, optional, defaults to the `current date`, it uses the little endian format (day, month, year).

```EBNF
Date ::= Day Month Year

Month ::= "1" #'[0-2]' | "0" NaturalDigit
Day ::= '0' #'[1-9]' | #'[1-2]' Digit | '3' #'[0-1]'
Year ::= Digit Digit

Natural ::= NaturalDigit Digit*
NaturalDigit ::= #'[1-9]'
Digit ::= "0" | NaturalDigit 
```

Examples:

* `030521` => March the 3rd, 2021
* `311222` => December the 31st, 2022

#### Tags

To label transactions, optional. For convenience it uses the hashtag format.

```EBNF
HashTag ::=  ('#' | '.')  Word

EOL ::= '\r'? '\n' 
Word ::= AlphaNum+ [ (' ' | '\t')+ | EOL ]
AlphaNum  ::= #'[A-Za-z0-9_-]'
```

Examples:

* `#grocerie`
* `.cash`

## Summery

| Transaction  | January  | February | March | April | May | June | July | August | September | October | November | December | Total |
|--------------|----------|----------|-------|-------|-----|------|------|--------|-----------|---------|----------|----------|-------|
| Income #1    | 1000     | 1200     | 800   | 0     | 0   | 0    | 0    | 1500   | 900       | 1100    | 1300     | 1000     |   0   |
| Income #2    | 0        | 500      | 0     | 0     | 800 | 0    | 0    | 0      | 0         | 0       | 0        | 0        |   0   |
| Expense #1   | -100     | -150     | -200  | 0     | 0   | -120 | 0    | -180   | 0         | -150    | -200     | 0        |   0   |
| Expense #2   | -20      | -800     | -800  | -800  | 0   | 0    | -900 | 0      | -1000     | 0       | 0        | -800     |   0   |

Total Income: **1500**  
Total Expenses: **-950**  
Remaining Budget: **550**

## Dependencies

* [clap-rs/clap](https://github.com/clap-rs/clap) - A full featured, fast Command Line Argument Parser for Rust

## Useful links

* [docs.rs](https://docs.rs/)
* [mermaid.live](https://mermaid.live/edit)
* [crates.io](https://crates.io/)
* [lib.rs](https://lib.rs/)

## License

[MIT](LICENSE)

## Author Information

[Mislav Matoković](https://github.com/mmatokovic)
