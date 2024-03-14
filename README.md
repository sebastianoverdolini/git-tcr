# TCR (Test && Commit || Revert)

## Introduction
**TCR (Test && Commit || Revert)** is a disciplined approach to software 
development aimed at ensuring code quality and integrity. 
It enforces the practice of running tests before committing 
changes to the codebase. If the tests pass, the changes are committed; 
otherwise, the changes are reverted to maintain a working state of the code.

## Requirements
To utilize TCR effectively, ensure you have the following prerequisites installed:

- **Git**: Version control system for tracking changes in your codebase.
- **sh**: Unix shell for executing commands within the TCR workflow.

## Installation
### Cargo
```
cargo install --git https://github.com/sebastianoverdolini/git-tcr
```

### Binaries
#### macOS (x86_64)
```
curl -L https://github.com/sebastianoverdolini/git-tcr/releases/latest/download/git-tcr-x86_64-apple-darwin > git-tcr && \
    chmod +x git-tcr && \
    sudo mv git-tcr /usr/local/bin/git-tcr
```

#### macOS (aarch64)
```
curl -L https://github.com/sebastianoverdolini/git-tcr/releases/latest/download/git-tcr-aarch64-apple-darwin > git-tcr && \
    chmod +x git-tcr && \
    sudo mv git-tcr /usr/local/bin/git-tcr
```

## Configuration
To configure TCR for your project, follow these steps:

1. **Place a `tcr.yaml`** configuration file 
    in the root directory of your project.

    ```yaml
    test: "<your test command>"
    before:
        - "<first command>"
        - "<second command>"
        - ...
    ```

    If you prefer not to define any commands to run before testing, 
    leave the `before` section as an empty array `[]`. 
    Commands specified here will be executed before running 
    the test command but won't trigger a revert action if they fail.

## Usage
To integrate TCR into your development workflow, follow these simple steps:

1. **Execute TCR**: Run the following command in your terminal:

    ```
    git tcr
    ```

    This command will automatically trigger the TCR process, 
    running tests and committing changes if they pass, 
    or reverting changes if the tests fail.

## Conclusion
By adopting TCR in your software development workflow, you can streamline your 
development process, minimize errors, and deliver high-quality code 
consistently. Embrace the TCR philosophy to build robust 
and reliable software with confidence.


