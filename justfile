alias b := build
alias c := clean
alias d := develop
alias f := format

# List commands
default:
    @just --list

# Check
check:
    cargo check

# Build
build: check
    trunk build --release

# Format
format: check
    cargo +nightly fmt
    # Format Leptos macros and tailwind classes 
    leptosfmt --experimental-tailwind $(fd --type file --extension rs)

# Clean
clean:
    cargo clean --verbose && rm -vr ./dist ./style/style.css

# Serve web app for development
develop:
    trunk serve --open --port 8290
