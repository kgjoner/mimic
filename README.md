# Mimic

![mimic](assets/img/mimic.jpg)

A CLI to store files and folders commonly used in several projects, helping to manage them and to replicate them into new projects.

## Installation

Clone this repo:

```
git clone https://github.com/kgjoner/mimic.git
```

Then, run inside project directory:

```
cargo install --path .
```

## Basic Usage

Let's say you use a same button component across projects. So you may open Project A, which has the
button, and run:

```
mimic swallow src/components/button
```

Then, in your next project requesting the button, you simply run:

```
mimic spit src/components/button
```

## Named Treasures

Each file or folder swallowed by Mimic is called a treasure. To avoid typing a long path each time you need to get it or to update it, you may let Mimic memorize treasure names.

```
mimic treasure name ui/button src/components/button
```

Now, you can run `swallow` and `spit` commands with the treasure name instead:

```
mimic spit ui/button
```

To see all memorized treasures, run:

```
mimic treasure list
```

## Help

To check all commands and options, run:

```
mimic help
```
