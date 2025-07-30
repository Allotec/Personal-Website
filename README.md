# Matthew Champagne Personal Website

A CSR website leveraging:
[Leptos](https://github.com/leptos-rs/leptos),
[Trunk](https://github.com/trunk-rs/trunk) and
[Tailwind](https://github.com/tailwindlabs/tailwindcss)

Template from [leptun](https://github.com/lpnh/leptrun)

Icons:
[HeroIcons](https://heroicons.com/outline)
[FlowBite](https://flowbite.com/icons/)
[Icons8](https://icons8.com/)
[Web Icon](https://icons8.com/icon/LUbIEvW7Ltnu/microprocessor)

Heavily inspired by [Aman Kumar](https://amankumar.ai/)

## Setup

### Compiling to WebAssembly

Install the Wasm target:

```sh
rustup target add wasm32-unknown-unknown
```

To check the installed targets:

```sh
rustup target list --installed
```

### Trunk

Install Trunk:

```sh
cargo install trunk --locked
```

_For additional installation options, refer to the [install
section](https://trunkrs.dev/#install) on Trunk's website_

### Cargo-make

Install cargo-make:

```sh
cargo install --force cargo-make
```

_For additional installation options, refer to the [installation
section](https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#installation)
on cargo-make repo_

## Usage

Feel free to leverage the tasks available in the `Makefile.toml`.

**Development**

```sh
cargo make run-dev
```

**Release**

```sh
cargo make run-release
```

**Updating `gh-pages` branch**

```sh
cargo make update-pages
```
