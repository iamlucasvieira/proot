# 🌳 PRoot

[![Unit Tests](https://github.com/iamlucasvieira/proot/actions/workflows/rust.yml/badge.svg)](https://github.com/iamlucasvieira/proot/actions/workflows/rust.yml)
[![GitHub License](https://img.shields.io/github/license/iamlucasvieira/proot)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/proot)](https://crates.io/crates/proot)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/proot)](https://crates.io/crates/proot)

PRoot is a GitHub PR Graph Visualizer that helps you seeing the relationships and dependencies among pull requests within a GitHub repository.

## Features

- **Visualize PR Dependencies:** Generate a graphical representation of pull request dependencies within a repository.
- **Open PR in Browser:** Open a specific pull request in the browser directly from the CLI.
- **Filter PRs to Open:** Filter PRs to open based on their dependencies.
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

Open a PR in the browser from the CLI:

```bash
proot <pr_number>
```

Filter PRs to open based on their dependencies:

```bash
proot filter --me
```

You can also use custom filters:

```bash
proot filter -c "is:closed"
```

To check if GitHub CLI is installed, run:

```bash
proot --check
```
