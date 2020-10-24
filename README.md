<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment commands-show-output -->

# Autofactory

<p align="center">
<a href="https://discord.gg/GCz7KgG">
    <img src="https://img.shields.io/discord/726947023231647798.svg?logo=discord&colorB=7289DA">
</a>
<a href="https://github.com/iMplode-nZ/Autofactory/actions">
    <img src="https://img.shields.io/github/workflow/status/iMplode-nZ/Autofactory/Rust">
</a>
<a href="https://github.com/iMplode-nZ/Autofactory/blob/main/LICENSE.md">
    <img src="https://img.shields.io/badge/license-GPL--3.0-brightgreen">
</a>
<a href="https://app.fossa.com/projects/git%2Bgithub.com%2FiMplode-nZ%2FAutofactory?ref=badge_shield" alt="FOSSA Status"><img src="https://app.fossa.com/api/projects/git%2Bgithub.com%2FiMplode-nZ%2FAutofactory.svg?type=shield"/></a>
</p>
<p align="center">
    <strong> <a href="https://youxplode.com/md.html?Autofactory">About</a> </strong>
</p>


[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FiMplode-nZ%2FAutofactory.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FiMplode-nZ%2FAutofactory?ref=badge_large)

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

There is also a very simple prototype<sup>[2](#fn2)</sup> that can be accessed by simply opening `file:///path/to/Autofactory/demo/index.html`. The space button can be used to pause the "game". Other controls are shown in the `Player 0's Resources ...` section.

---

<a name="fn1">1</a>: Install `pnpm` using `npm install --global pnpm`.

<a name="fn2">2</a>: Nothing in this will be kept; its purely for testing purposes.