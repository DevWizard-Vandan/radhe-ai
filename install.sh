#!/bin/bash
set -e
echo "Installing Radhe AI — Offline Terminal Assistant for Students"
RADHE_DIR="$HOME/.radhe"
BIN_DIR="$RADHE_DIR/bin"
MODELS_DIR="$RADHE_DIR/models"
# 1. Create directories
mkdir -p "$BIN_DIR" "$MODELS_DIR"
# 2. Download radhe binary (Linux)
RADHE_URL="https://github.com/DevWizard-Vandan/radhe-ai/releases/latest/download/radhe"
echo "Downloading radhe binary..."
curl -L "$RADHE_URL" -o "$BIN_DIR/radhe"
chmod +x "$BIN_DIR/radhe"
# 3. Download latest llama.cpp Linux CPU build
echo "Fetching latest llama.cpp release..."
LLAMA_RELEASE=$(curl -s https://api.github.com/repos/ggml-org/llama.cpp/releases/latest)
LLAMA_URL=$(echo "$LLAMA_RELEASE" | grep -o '"browser_download_url": *"[^"]*ubuntu-x64[^"]*"' | grep -o 'https://[^"]*' | head -1)
if [ -z "$LLAMA_URL" ]; then
  echo "Error: Could not find llama.cpp Linux build."
  echo "Hint: Check https://github.com/ggml-org/llama.cpp/releases for available builds."
  exit 1
fi
echo "Downloading llama.cpp: $LLAMA_URL"
curl -L "$LLAMA_URL" -o /tmp/llama.tar.gz
mkdir -p /tmp/llama_extract
tar -xzf /tmp/llama.tar.gz -C /tmp/llama_extract
find /tmp/llama_extract -name "llama-cli" -o -name "llama-completion" | xargs -I{} cp {} "$BIN_DIR/" 2>/dev/null || true
find /tmp/llama_extract -name "*.so" | xargs -I{} cp {} "$BIN_DIR/" 2>/dev/null || true
rm -rf /tmp/llama.tar.gz /tmp/llama_extract
# 4. Download Qwen2.5-Coder 1.5B model
MODEL_URL="https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf"
MODEL_DEST="$MODELS_DIR/Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf"
echo "Downloading Qwen2.5-Coder 1.5B model (~1GB)..."
curl -L "$MODEL_URL" -o "$MODEL_DEST"
# 5. Add BIN_DIR to PATH in shell config
SHELL_RC="$HOME/.bashrc"
if [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ] || [ "$SHELL" = "/usr/bin/zsh" ]; then
  SHELL_RC="$HOME/.zshrc"
fi
if ! grep -q "$BIN_DIR" "$SHELL_RC" 2>/dev/null; then
  echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_RC"
  echo "Added $BIN_DIR to PATH in $SHELL_RC"
else
  echo "$BIN_DIR already in PATH."
fi

# Install default packs
mkdir -p "$HOME/.radhe/packs"
for pack in math cs science; do
  curl -fsSL "https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/packs/$pack.md" \
    -o "$HOME/.radhe/packs/$pack.md"
  echo "  Downloaded $pack.md"
done
echo "Starter packs installed to ~/.radhe/packs/"

echo ""
echo "Radhe AI v0.6.0 installed successfully!"
echo "Restart your terminal, then try:"
echo "  radhe --explain \"binary search\""
echo "  radhe --summarize notes.txt"
echo "  radhe doctor"
