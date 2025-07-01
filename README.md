# mood-msg

Generate witty, mood-based git commit messages from your terminal, powered by local LLMs (Ollama) or a local fallback database.

## ✨ Features
- Generate short, witty, sarcastic commit messages based on your mood
- Uses a local LLM (Ollama) if available, or falls back to a local JSON database
- Integrates with git globally: just use `git commit -m "mood-msg -- tired"`
- Easy installation and setup

## 🚀 Installation

### 1. Install Rust (if you don't have it)
[Install Rust](https://www.rust-lang.org/tools/install)

### 2. Install mood-msg globally
From crates.io (recommended):
```sh
cargo install mood-msg
```
Or from the repo:
```sh
cargo install --path .
```

### 3. (Recommended) Run the install script to set up the global git hook
```sh
bash install.sh
```
This will:
- Set up a global git hook so you can use `git commit -m "mood-msg -- tired"` in any repo
- Ensure your CLI is in your PATH

## 🛠️ Manual Global Git Hook Setup
If you want to set up the hook manually:
```sh
mkdir -p ~/.git-templates/hooks
cat > ~/.git-templates/hooks/commit-msg << 'EOF'
#!/bin/bash
msg=$(cat "$1")
if [[ "$msg" =~ ^mood-msg\ --\ ([a-zA-Z]+)$ ]]; then
    mood="${BASH_REMATCH[1]}"
    new_msg=$(mood-msg "$mood")
    new_msg=$(echo "$new_msg" | sed 's/^\"//;s/\"$//')
    echo "$new_msg" > "$1"
fi
EOF
chmod +x ~/.git-templates/hooks/commit-msg
git config --global core.hooksPath ~/.git-templates/hooks
```

## 📝 Usage

### Generate a commit message for a mood
```sh
mood-msg tired
```

### Use with git (after hook setup)
```sh
git commit -m "mood-msg -- tired"
```
This will automatically replace the commit message with a witty, mood-based message.

## 💡 How it works
- Tries to generate a message using a local Ollama LLM (e.g., mistral)
- If Ollama is unavailable, falls back to a local `moods.json` file
- Prints the message in a fun, colored style

## 📦 Fallback: moods.json
If you want to customize or expand the fallback messages, edit the `moods.json` file in your project directory.

## 🧑‍💻 Contributing
Pull requests and issues are welcome!

## 📄 License
MIT 