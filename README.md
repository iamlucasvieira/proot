# 🌳 PRoot

PRoot is a GitHub PR Graph Visualizer that helps you seeing the relationships and dependencies among pull requests within a GitHub repository.

## Features

- **Visualize PR Dependencies:** Generate a graphical representation of pull request dependencies within a repository.
- **Check GitHub CLI Installation:** Quickly verify if GitHub CLI is installed on your system.

## Installation

To use this tool, you must have Rust installed on your machine. Follow these steps to install Rust if you haven't already: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Clone this repository to your local machine using:

```bash
git clone https://github.com/iamlucasvieira/proot
cd proot
```

Install it using Cargo:

```bash
cargo install --path .
```

## Usage

To visualize the PR dependencies, run:

```bash
proot
```

To check if GitHub CLI is installed, run:

```bash
proot --check
```
