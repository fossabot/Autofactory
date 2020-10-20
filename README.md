<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment commands-show-output -->

# Autofactory

Please read for an explanation of what this is: https://youxplode.com/md.html?Autofactory

## Compiling

To compile the Rust part of the program, run

```bash
> cd native
> cargo build
```

This assumes that you have [`rustup`](https://rustup.rs/) installed and a nightly version of the compiler as the default.

To compile the JavaScript part of the program, and link it with the Rust part, run<sup>[1](#fn1)</sup>

```bash
> pnpm run build
```

Start using

```bash
> pnpm run start
```

## Prototype

There is also a very simple prototype<sup>[2](#fn2)</sup> that can be accessed by simply opening `file:///path/to/NE/demo/index.html`. The space button can be used to pause the "game". Other controls are shown in the `Player 0's Resources ...` section.

---

<a name="fn1">1</a>: Install `pnpm` using `npm install --global pnpm`.
<a name="fn2">2</a>: Nothing in this will be kept; its purely for testing purposes.
