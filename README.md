# Squooshies

---


Squooshies is a tentative to add a visual for converting massive amounts of pictures and is named after Squoosh by Google who does
the same for a single picture at a time.

Built on top of the Dioxus-desktop framework, this app is fully in Rust and aim to be performant as well as to look good.

## Installation
----

### Download

The latest release contains an executable for windows. 

### Build

To build this app you will need Rust installed and [Nasm](https://www.nasm.us/) as well.

The Dioxus framework comes with its own cli to build the app. 

```Powershell

cargo install dioxus-cli

```

Or if you have cargo-binstall

```Powershell

cargo-binstall dioxus-cli

```

Clone the repository 

```Powershell

git clone "https://github.com/SoapyDev/Squooshies.git"

```

You are ready to build the application.

```Powershell

cd Squooshies
dx serve --release

```
