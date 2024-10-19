# PRT: Pull request TUI

## Description
TUI for managing pull requests on GitHub.

## Installation

### Using Homebrew

Just tap the repository and install the application.
```
brew tap djego/prt
brew install prt
```

### build from source

Go to releases and download the latest version for your platform.

```
chmod a+x prt
sudo mv prt /usr/local/bin
```

## Usage

You should have a GitHub token in order to use this application (PAT)

You can create one [here](https://github.com/settings/tokens).

After that, you can run the application with the following command:
````bash
prt
````
Insert PAT and you are ready to go!

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
![prt demo](https://github.com/user-attachments/assets/dda30cbf-7e9f-47fe-b091-dbb1d630d4a8)

Enjoy creating pull requests from TUI with PRT 🚀!!
