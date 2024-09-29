# PRT: Pull request TUI

## Description
TUI for managing pull requests on GitHub.

## Installation

## Development

For easy development, first clone the repository:
````bash
git clone https://github.com/djego/prt.git
cd prt
````

Then, create a `.env` file with the following content:
````bash
GITHUB_TOKEN=your_github_token
GITHUB_DEFAULT_BRANCH=your_github_default_branch # if not set, it will be 'main'
````
Finally, use the following commands:
````rust
cargo updatae
cargo run
````

## Demo
[![asciicast](https://asciinema.org/a/677701.svg)](https://asciinema.org/a/677701)

Happy coding!
