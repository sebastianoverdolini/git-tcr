# TCR (Test && Commit || Revert)

## Introduction
**TCR (Test && Commit || Revert)** is a disciplined approach to software 
development that encourages taking small, incremental steps. 
The rules are straightforward: if tests pass, the changes are committed; 
if they fail, the changes are reverted. 
This method promotes a cycle of continuous testing and feedback, 
helping developers stay focused on producing functional code at each step.
TCR was introduced by Kent Beck as part of "Limbo on the Cheap". 
You can read his original article [here](https://medium.com/@kentbeck_7670/test-commit-revert-870bbd756864).

## Requirements
- **Git**
- **sh**

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
    test: "<your test command>"
    before: # A list of commands to execute before the test command
        - "<first command>"
        - "<second command>"
        - ...
   no_verify: <true|false> # Optional: Set to 'true' to skip verification steps. Default is false.
    ```

    If you prefer not to define any commands to run before testing, 
    leave the `before` section as an empty array `[]`. 
    Commands specified here will be executed before running 
    the test command but won't trigger a revert action if they fail.

## Usage
```
git tcr
```

## Conclusion
By adopting TCR in your software development workflow, you can streamline your 
development process, minimize errors, and deliver high-quality code 
consistently. Embrace the TCR philosophy to build robust 
and reliable software with confidence.


