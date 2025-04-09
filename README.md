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
    commit:
      no_verify: <true|false>
      # Optional: Includes --no-verify option. Default is false.
    ```

### AI Powered Automatic Commit Message

TCR can generate commit messages automatically using an AI model. The AI model uses the changes in your Git diff to create a descriptive commit message.

#### Installation of Ollama and Mistral

To enable AI-powered commit messages, you need to install **Ollama** and the **Mistral** model locally.

1. **Install Ollama**:
   Ollama is a tool to run various AI models locally. You can download it from the official website:
    - [Ollama Download](https://ollama.com/download)

2. **Install Mistral Model**:
   After installing Ollama, you need to download the Mistral model. Run the following command to install it:
    ```bash
   ollama pull mistral
    ```
3. **Ensure that Ollama is available in your PATH: Verify the installation by running:**
    ```bash
    ollama --version
    ```
Once Ollama and Mistral are installed, the TCR tool will automatically use them to generate commit messages based on your Git diff.

If Ollama or Mistral is not available, TCR fallbacks to using a default commit message, "WIP".

## Usage
```
git tcr
```

### Watch Mode
You can also run TCR in **watch mode** to automatically run it
whenever a file is changed. To enable this mode, use the `--watch` flag:

```
git tcr --watch
```

The git-ignored files and the `.git` directory are
automatically excluded from the watch.

## Conclusion
By adopting TCR in your software development workflow, you can streamline your
development process, minimize errors, and deliver high-quality code
consistently. Embrace the TCR philosophy to build robust
and reliable software with confidence.
