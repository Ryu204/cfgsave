# Description

**cfgsave** is a tool to copy files into a common directory.

It was developed for config files management in Linux filesystem ([dotfiles](https://wiki.archlinux.org/title/Dotfiles)).

The tool is however designed to work with a version control system (VCS), such as **Git**. By manually copying files into a common directorty, **cfgsave** reduces the need to rely on symlinks. The user only has to manage VCS inside one directory.

# Usage

## Options
There are 5 possible commands:
1. Add a file into the directory
```sh
$ cfgsave add <filename>
```
2. Remove a file from the directory
```sh
$ cfgsave remove <filename>
```
3. List files currently tracked
```sh
$ cfgsave list
```
4. Capture live status of tracked files and refelct them in the directory
```sh
$ cfgsave snap
```
5. Use data stored inside the directory to overwrite real files
```sh
$ cfgsave publish [quiet]
```
Use `quiet` if you want to overwrite all current tracked files instead of reviewing them one by one.

## Used with Git

The common directory is `$HOME/.cfgsave`. 

The backup process is like following:

```sh
$ cfgsave snap
$ cd $HOME/.cfgsave
$ git add -A
$ git commit -m "`date +\"%Y-%m-%d\"`"
```
