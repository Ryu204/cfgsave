# Description

**cfgsave** is a tool to copy files into a common directory.

It was developed for managing configuration files in the Linux filesystem ([dotfiles](https://wiki.archlinux.org/title/Dotfiles)).

The tool is however designed to work with a version control system (VCS), such as **Git**. By manually copying files into a common directory, **cfgsave** reduces the need to rely on symlinks. The user only has to manage VCS inside one directory.

Since this tool is way too simple, it's recommended to only manually track files the user **explicitly** edits.

# Usage

## Options
There are 5 possible commands:
1. Add a file to the directory
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
4. Capture the live status of tracked files and reflect them in the directory
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

Initialization steps:
1. Create a git repo in `$HOME/.cfgsave`
1. Configure an origin
1. (Optional) Ensure no other applications write to `$HOME/.cfgsave`

Then, you can create a somewhat script for backing up the data like this:

```sh
$ cfgsave snap
$ cd $HOME/.cfgsave
$ git add -A
$ git commit -m "`date +\"%Y-%m-%d\"`"
$ git push origin
```

## Restore

Use `cfgsave publish` and carefully review which files should be pasted to their original address.

Please note that since all the data is located under `$HOME`, root user will not be able to use another user's data. If you want to `publish` to a system file by `sudo`, use the `-E` flag:

```sh
$ sudo -E cfgsave publish
```

Tracked files inside a home directory will be copied to `$HOME` regardless of the previous creator of the directory. This comes in handy when one wants to back up user configs to another account.

# Limitations

* The tool is developed for personal use of the writer. So the features are really limited.
* If `$HOME` is not properly defined when the application is called, it will not be able to find the home directory of the current user.
