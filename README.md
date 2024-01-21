# Budget CLI

> **Welcome from Budget CLI, your go-to command-line interface for personal finance management**

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

## Install

Add `bud` to path.

```Bash
|> cargo install --path .
```

## Examples

To `get` budget or transactions, summery or goals:

* view summery report, you can filter by `--id`, `--name` or `--tag`.

    ```Bash
    |> bud get summery
    ```

* view goals, you can filter by `--id`, `--name` or `--tag`..

    ```Bash
    |> bud get goals 
    ```

To `set` Financial Goals by `--tag`, `--name` and `--amount`

```Bash
|> bud set Croatia 1000€ 100320 1m12x .travel
```

To `edit` a single transaction use the command `bud edit`:

```Bash
|> bud edit --id Netflix 12€ 100320 1m12x .movies .covid
```

To `del` a single transaction use the command `bud del`:

```Bash
|> bud del --id
```

To `add` a single transaction use the command `bud add` by `--name*`, `--amount*`, `--startdate`, `--enddate`, `--tag` and `--type`

* **Start date** defaults to `current date`
* **End date**: defaults to `current date`
* **Operation** defaults to `expense` but you can set it as `income`:

```Bash
|> bud add Salary 1300€ 1m --type
|> bud add Netflix 7.99€ 100320 1m12x .subscription
```

A transaction as the properties:

* **Name**: a descriptive name
* **Amount**: the amount in monetary currency
* **Start date**: the start date of transaction
* **End date**: a duration that the transaction applies to
* **Tags**: for grouping transactions
* **Operation**: a transaction type `expense` or `income`, defaults to expense

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

## Machine-readable output

**For more complex structured data, programs should accept a flag to provide output (e.g. `--output-format`, or `--message-format` if many lines of structured data are printed out).**

* Programs should support at least json machine-readable output.
* Programs may also provide their output as CSV or other self-describing format (markdown table).
* A self-describing format is one where the keys, or some equivalent, are part of the serialized output.
* Formats like [protobuf](https://protobuf.dev/) are suitable as well, if up-to-date IDLs (e.g. `.proto` files) are published along with releases. One neat trick is to embed them into your binary and add a command to write them out to a given directory.
* If many lines of structured data are incrementally printed out, prefer a format like [newline-delimited JSON](https://ndjson.org/). This is the format used by Cargo's `--message-format` json option.

### Markdown

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
* [Command Line Interface Guidelines](https://clig.dev/)
* [mermaid.live](https://mermaid.live/edit)
* [crates.io](https://crates.io/)
* [lib.rs](https://lib.rs/)
* [Rain's Rust CLI recommendations](https://rust-cli-recommendations.sunshowers.io/index.html)

## License

[MIT](LICENSE)

## Author Information

[Mislav Matoković](https://github.com/mmatokovic)
