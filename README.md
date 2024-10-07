# PRT: Pull request TUI

## Description
TUI for managing pull requests on GitHub.

## Installation
Go to releases and download the latest version for your platform.

```
chmod a+x prt
sudo mv prt /usr/local/bin
```

## Usage
You should have a GitHub token in order to use this application (PAT)

You can create one [here](https://github.com/settings/tokens).


## Development

For easy development, first clone the repository:
````bash
git clone https://github.com/djego/prt.git
cd prt
````
Then, install the following dependencies and execute application:
````bash
cargo update
cargo run
````

## Demo
[![asciicast](https://asciinema.org/a/679322.svg)](https://asciinema.org/a/679322)

Enjoy creating pull requests with ease!
