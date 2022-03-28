<!-- This doc uses c for solar syntax, because it works rather nicely with treesitter for markdown -->
# Changes

## Semantik

Include interfaces.


## Syntax

### Types/Functions

*use `fun` keyword to describe lambda types*

Instead of
```kotlin
(String, String) => String => Void
```

Use

```kotlin
fun(String, String) => fun(String)
```

Reason:
> - consistent with closure syntax
> - consistent with function argument syntax
> - easy to read and to type
> - less weird special characters
> - don't have to type awkward `void` thing


### Function Syntax

*Replace the `let` keyword with `fun`*

Instead of
```kotlin
let reduce(list, f, initial) = when list
    is [] => initial
    is [elem, ..rest] => reduce rest f (f initial elem)
```

Use

```kotlin
fun reduce(list, f, initial) = when list
    is [] => initial
    is [elem, ..rest] => reduce rest f (f initial elem)
```

```kotlin
fun reduce(
    list :: List b,
    f :: fun (a, b) => a,
    initial :: a,
    ) = when list
        is [] => initial
        is [elem, ..rest] => reduce rest f (f initial elem)
```

Reason:
> this way it is more obvious, what kinds of functions are in the global scope

### Closures

* use `fun` prefix for closures*

Instead of
```kotlin
(x) => x*x
```

Use

```kotlin
fun (x :: a) :: a => x*x
fun (x) => x*x
```

Reason:
> closures become more obvious and typing fun isn't hard.

### When

*Replace `then` keyword with `=>` in when statement*

Instead of
```kotlin
when list
    is [] then 0
    is [x, ..rest] then x + p rest
```

Use

```kotlin
when list
    is [] => 0
    is [x, ..rest] => x + p rest
````

Reason:
> Far easier to distinguish `=>` from yet another 'special identifier' (`then`)
