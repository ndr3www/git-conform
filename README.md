<p align="left">
  <a href="https://crates.io/crates/git-conform">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/git-conform" />
    <img alt="Crates.io Downloads" src=https://img.shields.io/crates/d/git-conform />
  </a>
</p>

## 📖 About
`git-conform` is a simple git extension that helps you to keep track of the repositories on your local machine and their remote counterparts.
It works by scanning your **home directory** (or just the ones you specified) in search for git repositories, and then storing their paths in
the tracking file located at `~/.local/share/git-conform`.

## ✨ Features
- Effortlessly integrates with the git ecosystem ↔️
- Single self-contained executable 🗄️
- Blazingly fast ⚡
- Highly memory-efficient ♻️
- Straightforward design ✅

## 💿 Installation

### Through cargo
`cargo install git-conform`

### Using the setup shell script
```bash
bash <(curl -sSL https://raw.githubusercontent.com/ndr3www/git-conform/main/setup.sh)
```

## 📋 List of available commands
- `git conform scan` - searches for untracked repositories and automatically adds them for tracking
- `git conform list` - prints the list of tracked repositories
- `git conform add` - adds repositories for tracking
- `git conform rm` - removes repositories from tracking
- `git conform check` - prints the following details about each repository: output of `git status -s` and ahead/behind commit metrics between local and remote branches

**Type `git conform --help`, `git conform -h` or `git conform help` to get more details**
