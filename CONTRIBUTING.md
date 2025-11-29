# Contributing

This document provides a lightweight, consistent workflow for this repository.

## 1. Prerequisites

Install the following tools:

- Git  
- VS Code (recommended)  
- **uv** (package and environment manager)  
- pre-commit (installed via uv)
- Rust (verify with `rustc --version`)
- [VS Code Extension: C# dev kit by Microsoft](https://learn.microsoft.com/en-us/dotnet/core/install/windows#install-with-visual-studio-code)
- [Build Tools for VS 2022 (as needed)](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
- Java

## 2. Fork and Clone

1. Fork the repository on GitHub.  
2. Clone your fork and open it in VS Code.

```
git clone https://github.com/YOUR_USERNAME/REPO_NAME.git
cd REPO_NAME
```

## 3. One-Time Setup

Create a local environment and install dependencies.

```shell
uv python pin 3.12
uv venv
.venv/Scripts/activate        # Windows
# source .venv/bin/activate   # Mac or Linux

uv sync --extra dev --extra docs --upgrade
uv run pre-commit install
```

In Git Bash / bash / zsh, test rust until no errors and all tests pass:

```shell
rustc --version
cargo test -- --nocapture
cargo fix --lib -p cep-relationship #as needed
cargo test -- --nocapture
```

## 4. Validate Changes

Run standard local checks.

```
git pull origin main
uvx ruff check . --fix
uvx ruff format .
uvx deptry .
uv run pyright
uv run pytest
uvx pre-commit autoupdate
uvx pre-commit run --all-files
```

## 5. Building Package and/or Docs

```
uv build
uv run python tools/sync_schemas_for_docs.py
uv run mkdocs build --strict
uv run mkdocs serve
```

## 6. Commit and Push

```
git add .
git commit -m "Your message"
git push -u origin main
```

## 7. Open a Pull Request

Open a PR from your fork to the `main` branch of the target repository.

Guidelines for good PRs are here:  
`REF_PULL_REQUESTS.md`

---

If you have questions, open an issue in the target repository.  
Thank you for contributing.
