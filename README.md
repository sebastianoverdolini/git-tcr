# TCR (Test && Commit || Revert)

## Introduction
**TCR (Test && Commit || Revert)** is a variation of test-driven development
that forces taking small, incremental steps. 
The rules are straightforward: if tests pass, the changes are committed; 
if they fail, the changes are reverted. 
This method promotes a cycle of continuous testing and feedback, 
helping developers stay focused on producing functional code at each step.
TCR was introduced by Kent Beck as part of "Limbo on the Cheap". 
You can read his original article [here](https://medium.com/@kentbeck_7670/test-commit-revert-870bbd756864).

## Requirements
- **Git**

## Installation
### Binaries
#### Apple Silicon
```
curl -L https://github.com/sebastianoverdolini/git-tcr/releases/latest/download/git-tcr-aarch64-apple-darwin > git-tcr && \
    chmod +x git-tcr && \
    sudo mv git-tcr /usr/local/bin/git-tcr
```
### Cargo
```
cargo install --git https://github.com/sebastianoverdolini/git-tcr
```

### Man page (optional)
Since `git tcr` is a git subcommand, `git tcr --help` opens git's own manual
lookup (`git help tcr`) rather than the binary's built-in `-h`/`--help`.
Installing the man page makes `git tcr --help` work as expected:
```
sudo mkdir -p /usr/local/share/man/man1 && \
    curl -L https://github.com/sebastianoverdolini/git-tcr/releases/latest/download/git-tcr.1 \
        | sudo tee /usr/local/share/man/man1/git-tcr.1 > /dev/null
```
(`-h` works out of the box either way — this only affects the long `--help` form.)

## Configuration
To configure TCR for your project, place a `tcr.yaml` configuration file
in the root directory of your project. You can create it interactively:

```
git tcr init
```

This asks for the test command(s) to run (e.g. `npm test`) and whether
commits should skip git hooks with `--no-verify`, then writes `tcr.yaml`
for you. If a `tcr.yaml` already exists, it asks before overwriting it.

Each test command is actually run before being kept: one that fails to
run successfully is rejected automatically, and you're asked to type it
again. One that runs successfully is still shown to you for confirmation,
in case it ran but wasn't the command you meant.

Alternatively, write the file by hand:

```yaml
version: 1 # Optional: the tcr.yaml schema version. Defaults to 1 if omitted.
test:
    program: <...> # e.g "cargo"
    args: [...]    # e.g ["test"]
no_verify: <true|false> # Optional: Set to 'true' to skip verification steps. Default is false.
```

`version` declares which `tcr.yaml` shape the file uses. It only changes
when the format itself changes in a backward-incompatible way, so most
releases of `git-tcr` won't require bumping it. If a config declares a
`version` newer than the installed `git-tcr` understands, it refuses to
run and asks you to upgrade instead of misreading the file.

To run multiple test commands, declare `test` as a list instead. 
Commands run in order and stop at the first failure:

```yaml
test:
    - program: "tsc"
      args: ["--noEmit"]
    - program: "npm"
      args: ["run", "test"]
```

## Usage
```
git tcr [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-m, --message <MESSAGE>` | Use a custom commit message instead of the default `WIP`. |
| `--trailer <TRAILER>` | Append a git trailer to the commit message (e.g. `Issue: GDT-42`). Can be repeated multiple times. |

### Examples

Run TCR with the default `WIP` commit message:
```
git tcr
```

Run TCR with a custom commit message:
```
git tcr -m "feat: add login page"
```

Run TCR with a custom message and one or more trailers:
```
git tcr -m "feat: add login page" --trailer "Issue: GDT-42" --trailer "Reviewed-by: Alice"
```


## Conclusion
By adopting TCR in your software development workflow, you can streamline your 
development process, minimize errors, and deliver high-quality code 
consistently. Embrace the TCR philosophy to build robust 
and reliable software with confidence.
