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

## Configuration
To configure TCR for your project, follow these steps:

1. **Place a `tcr.yaml`** configuration file 
    in the root directory of your project.

    ```yaml
    test:
        program: <...> # e.g "cargo"
        args: [...]    # e.g ["test"]
    no_verify: <true|false> # Optional: Set to 'true' to skip verification steps. Default is false.
    ```

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
