<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment commands-show-output -->

# NE

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
pnpm run start
```

---

<a name="fn1">1</a>: Install `pnpm` using `npm install --global pnpm`.
