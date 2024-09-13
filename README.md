# ezcfg

Help you manage various configuration files in your system, like `.nvim`、`.zshrc` and etc.

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
    ["zsh/.zshrc", "~/.zshrc"],
    ["nvim",       "~/.config/nvim"],
]
```

Now just run the command.

```sh
ezcfg
```