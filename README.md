# ezcfg

Help you manage various configuration files in your system, like `.nvim`、`.zshrc` and etc.

> 🚧 **Work in Progress**
>
> ezcfg is currently in active development...

## Install

### unix

```bash
brew tap tim101010101/ezcfg
brew install ezcfg
```

### windows

**Not supported for the time being.**

## Usage

Find a place to centrally manage your configuration files. 

For example, I choose `~/.dotfiles/`.

```sh
cd ~
mkdir .dotfiles
```

Create a configuration file here.

```sh
touch .ezcfg.toml
```

Configure the source path and target path of the link you need.

```toml
links = [
    ["zsh/.zshrc", "$HOME/.zshrc"],
    ["nvim",       "$HOME/.config/nvim"],
]
```

Now just run the command.

```sh
ezcfg
```