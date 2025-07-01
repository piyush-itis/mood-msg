#!/bin/bash
set -e

# 1. Install mood-msg globally if not present
if ! command -v mood-msg &> /dev/null; then
    echo "Installing mood-msg globally using cargo..."
    if cargo install mood-msg; then
        BIN_PATH="$(which mood-msg)"
    else
        echo "Trying to install from local path..."
        cargo install --path .
        BIN_PATH="$(which mood-msg)"
    fi
else
    BIN_PATH="$(which mood-msg)"
    echo "mood-msg is already installed at $BIN_PATH."
fi

# 2. Ensure ~/.cargo/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "Added ~/.cargo/bin to PATH in ~/.bashrc"
fi

# 3. Set up global git hooks directory
HOOKS_DIR="$HOME/.git-templates/hooks"
mkdir -p "$HOOKS_DIR"

# 4. Write the commit-msg hook with the full path to mood-msg
cat > "$HOOKS_DIR/commit-msg" << EOF
#!/bin/bash
msg=\$(cat "\$1")
if [[ "\$msg" =~ ^mood-msg\\ --\\ ([a-zA-Z]+)
$ ]]; then
    mood="\${BASH_REMATCH[1]}"
    new_msg=\"\$($BIN_PATH "\$mood")\"
    # Remove leading/trailing quotes if present
    new_msg=\$(echo "\$new_msg" | sed 's/^\"//;s/\"$//')
    echo "\$new_msg" > "\$1"
fi
EOF

chmod +x "$HOOKS_DIR/commit-msg"

git config --global core.hooksPath "$HOOKS_DIR"

echo "\nInstallation complete! You can now use:"
echo '  git commit -m "mood-msg -- tired"'
echo "in any repository, and your mood-based commit message will be generated automatically." 