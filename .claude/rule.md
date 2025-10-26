# Development Rules

## Code Editing Workflow

When editing Rust code in this project, you MUST follow these steps in order:

### 1. Edit Code
Make your changes to the source files.

### 2. Format Code (Required)
```bash
cargo fmt
```
This ensures consistent code formatting across the project.

### 3. Run Linter (Required)
```bash
cargo clippy
```
Review and address all warnings and errors reported by clippy.

### 4. Build Project (Required)
```bash
cargo build
```
Ensure the project compiles successfully without errors.

### 5. Run Tests (If Applicable)
```bash
cargo test
```
Run this command if tests exist in the project.

## Important Notes

- **NEVER** skip steps 2-4. These are mandatory quality checks.
- If clippy reports warnings, review them and fix critical issues before proceeding.
- If build fails, fix all compilation errors before moving forward.
- Always commit working code that passes all checks.

## Release Build

For production builds, use:
```bash
cargo build --release
```

This creates an optimized binary with:
- Maximum optimization level (opt-level = 3)
- Link-time optimization (LTO)
- Stripped symbols for smaller binary size

## Documentation

Update documentation when:
- Adding new public APIs
- Changing existing functionality
- Adding new CLI commands or options

## Commit Guidelines

Before committing:
1. ✓ Code is formatted (`cargo fmt`)
2. ✓ Linter passes (`cargo clippy`)
3. ✓ Project builds successfully (`cargo build`)
4. ✓ Tests pass (if applicable)
5. ✓ Documentation is updated
