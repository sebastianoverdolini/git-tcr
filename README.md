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

## Usage
```
git tcr
```

### Scribe
You can use the `--scribe` flag to automatically generate commit messages based on your staged changes.
This feature requires [Ollama](https://ollama.com/) running locally and `qwen2.5-coder:3b`.

Example:
```
git tcr --scribe
```
If `--scribe` is not provided, the default commit message is `WIP`.

## Conclusion
By adopting TCR in your software development workflow, you can streamline your 
development process, minimize errors, and deliver high-quality code 
consistently. Embrace the TCR philosophy to build robust 
and reliable software with confidence.
