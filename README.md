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

Create your profiles here, like `nvim` and `zsh/.zshrc`

And `ezcfg` needs a configuration file `.ezcfg.toml` to tell it how it works.

```sh
echo 'links = [
    ["zsh/.zshrc", "$HOME/.zshrc"],
    ["nvim",       "$HOME/.config/nvim"],
]' > .ezcfg.toml
```

At this time, our directory structure will probably be like this.

```sh
~/.dotfiles
├──nvim
│  ├──init.lua
│  ├──lazy-lock.json
│  └──lua
├──zsh
│  └──.zshrc
└──.ezcfg.toml
```

Now just run the command.

```sh
ezcfg
```

After the command is successfully executed, we can see that the soft links we need is already in the specified location.

```sh
~
├──.config
│  └──nvim -> ~/.dotfiles/nvim
└──.zshrc -> ~/.dotfiles/zsh/.zshrc
```