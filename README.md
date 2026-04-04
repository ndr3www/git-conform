<p align="left">
  <a href="https://crates.io/crates/git-conform">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/git-conform" />
    <img alt="Crates.io Downloads" src=https://img.shields.io/crates/d/git-conform />
  </a>
</p>

## About
`git-conform` is a simple git extension that helps you to keep track of the repositories on your local machine and their remote counterparts.
It works by scanning your **home directory** (or just the ones you specified) in search for git repositories, and then storing their paths in
the tracking file located at `~/.local/share/git-conform`. By typing `git conform check --all` you can see useful information
about all the repositories on your machine at once such as uncommitted changes or unsynced commits between local and remote branches.

## Installation

### Through cargo
`cargo install git-conform`

### Using the setup shell script
> [!IMPORTANT]  
> Before continuing, make sure you have `wget` and `curl` installed on your system
```bash
bash <(curl -sSL https://raw.githubusercontent.com/ndr3www/git-conform/main/setup.sh)
```

## Available commands and options
- `git conform scan` - searches for untracked repositories and automatically adds them for tracking
  - `-a, --all` - scan all directories in your /home
  - `--hidden` - allow scanning hidden directories
  - `-q, --quiet` - suppress information messages
<br></br>
- `git conform list` - prints the list of tracked repositories
<br></br>
- `git conform add` - adds specified repositories for tracking
<br></br>
- `git conform rm` - removes specified repositories from tracking
  - `-a, --all` - remove all repositories from tracking
<br></br>
- `git conform check` - inspects specified repositories
  - `-a, --all` - inspect all tracked repositories
  - `-s, --status` - print only the output of `git status -s`
  - `-r, --remotes` - print only the differences between local repositories and their remotes
<br></br>

Type `git conform help` or `git conform -h` to get more details. Also, you can get more comprehensive info about specific subcommand like this: `git conform help <subcommand>`.
