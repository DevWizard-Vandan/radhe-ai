#!/bin/bash
set -e
echo "Installing Radhe AI — Offline Terminal Assistant"
RADHE_DIR="$HOME/.radhe"
BIN_DIR="$RADHE_DIR/bin"
MODEL_DIR="$RADHE_DIR/models"
mkdir -p "$BIN_DIR" "$MODEL_DIR"
# Download radhe binary
curl -L "https://github.com/DevWizard-Vandan/radhe-ai/releases/latest/download/radhe" -o "$BIN_DIR/radhe"
chmod +x "$BIN_DIR/radhe"
# Download llama-completion (Linux CPU build)
LLAMA_RELEASE=$(curl -s https://api.github.com/repos/ggml-org/llama.cpp/releases/latest)
LLAMA_URL=$(echo "$LLAMA_RELEASE" | grep -o '"browser_download_url": "[^"]*ubuntu[^"]*"' | head -1 | cut -d'"' -f4)
curl -L "$LLAMA_URL" -o /tmp/llama.zip
unzip -o /tmp/llama.zip llama-completion -d "$BIN_DIR" 2>/dev/null || true
chmod +x "$BIN_DIR/llama-completion"
# Download model
curl -L "https://huggingface.co/Qwen/Qwen2.5-Coder-0.5B-Instruct-GGUF/resolve/main/qwen2.5-coder-0.5b-instruct-q4_k_m.gguf" -o "$MODEL_DIR/qwen2.gguf"
# Add to PATH
SHELL_RC="$HOME/.bashrc"
[[ "$SHELL" == *zsh* ]] && SHELL_RC="$HOME/.zshrc"
if ! grep -q '.radhe/bin' "$SHELL_RC"; then
    echo 'export PATH="$HOME/.radhe/bin:$PATH"' >> "$SHELL_RC"
fi
echo ""
echo "Radhe AI installed! Run: source ~/.bashrc"
echo "Then try: radhe --code 'hello world in c'"
