# Notes @ FutureMe

> Things to do in this project when I come back.

## Verify the list of bindings

- I am sure there are still some bindings that could be added: find them and add them.

## Binding variants

This crate currently supports one binding variant named `000` which means:

```text
 0 0 0
 ┬ ┬ ┬
 │ │ └── status flags passed as a separate argument
 │ └── rounding mode passed as a separate argument
 └── result returned by value
```

Implement other binding variants when feasible:

```text
 0 0 1
 ┬ ┬ ┬
 │ │ └── status flags is a global variable
 │ └── rounding mode passed as a separate argument
 └── result returned by value
```

```text
 0 1 0
 ┬ ┬ ┬
 │ │ └── status flags passed as a separate argument
 │ └── rounding mode is a global variable
 └── result returned by value
```

```text
 0 1 1
 ┬ ┬ ┬
 │ │ └── status flags is a global variable
 │ └── rounding mode is a global variable
 └── result returned by value
```

```text
 1 0 0
 ┬ ┬ ┬
 │ │ └── status flags passed as a separate argument
 │ └── rounding mode passed as a separate argument
 └── result is passed back by reference
```

```text
 1 0 1
 ┬ ┬ ┬
 │ │ └── status flags is a global variable
 │ └── rounding mode passed as a separate argument
 └── result is passed back by reference
```

```text
 1 1 0
 ┬ ┬ ┬
 │ │ └── status flags passed as a separate argument
 │ └── rounding mode is a global variable
 └── result is passed back by reference
```

```text
 1 1 1
 ┬ ┬ ┬
 │ │ └── status flags is a global variable
 │ └── rounding mode is a global variable
 └── result is passed back by reference
```

## Update documentation

Update descriptions of all bindings with code examples.

## Write user guide

Write user guide and link from documentation of this crate. 
