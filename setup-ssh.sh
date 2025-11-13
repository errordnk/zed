#!/bin/bash
# Setup SSH authentication for GitHub

echo "Setting up SSH key for GitHub..."

# Check if SSH key exists
if [ ! -f ~/.ssh/id_ed25519 ]; then
    echo "Generating SSH key..."
    ssh-keygen -t ed25519 -C "your_email@example.com" -f ~/.ssh/id_ed25519 -N ""
fi

echo ""
echo "Your public key (add this to GitHub):"
echo "https://github.com/settings/keys"
echo ""
cat ~/.ssh/id_ed25519.pub
echo ""

# Change remote to SSH
git remote set-url origin git@github.com:errordnk/zed.git

echo "Remote changed to SSH. After adding key to GitHub, push with:"
echo "git push -u origin claude/fix-ui-input-editor-stubs-011CV4z5KGGzFXqvUTkJGYxb"
