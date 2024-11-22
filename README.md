# Tsuki

#### What is it?
Tsuki is a framework (and accompanying runtime) for Lua.
The eventual end goal is to provide a feature-rich library of
functions to make Lua viable for general purpose programming,
similarly to what Node.js enables for JavaScript.

#### Why is this needed?
Lua, while great, was [never designed to be a "do-it-all" language](https://www.lua.org/pil/p1.html).
The libraries cover only the concepts covered by standard C, and so
the available standard libraries are missing a lot of filesystem
(in fact, the same concept of a filesystem doesn't exist) and I/O
concepts commonly found in other languages. Tsuki aims to leverage
Lua's extremely powerful extensibility to provide a more ergonomic
experience for these use cases.

---

## How does it work?
Tsuki's CLI is made in Rust and uses calls to Rust code for most
of its internal functions. When called on a file, it first creates its module
files if they don't already exist. It then reads in the source of the file
it was called on, creates a new Lua environment with Tsuki's internal functions
loaded in, and then executes the Lua file with an updated `package.path` to
include Tsuki's Lua libraries (which also include luals annotations).
