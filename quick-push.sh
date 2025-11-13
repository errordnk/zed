#!/bin/bash
# Quick push script with multiple options

echo "==================================="
echo "Push Zed Terminal to GitHub"
echo "==================================="
echo ""
echo "Choose authentication method:"
echo ""
echo "1) SSH Key (recommended)"
echo "2) Personal Access Token"
echo "3) Show manual instructions"
echo "4) Exit"
echo ""
read -p "Enter choice [1-4]: " choice

case $choice in
  1)
    echo ""
    echo "=== SSH Key Setup ==="
    if [ ! -f ~/.ssh/id_ed25519 ]; then
      echo "Generating SSH key..."
      read -p "Enter your GitHub email: " email
      ssh-keygen -t ed25519 -C "$email" -f ~/.ssh/id_ed25519 -N ""
    fi
    
    echo ""
    echo "Your PUBLIC KEY (copy this):"
    echo "------------------------------------------------------------"
    cat ~/.ssh/id_ed25519.pub
    echo "------------------------------------------------------------"
    echo ""
    echo "Steps:"
    echo "1. Copy the key above"
    echo "2. Go to: https://github.com/settings/keys"
    echo "3. Click 'New SSH key'"
    echo "4. Paste and save"
    echo ""
    read -p "Press Enter after adding key to GitHub..."
    
    # Change remotes to SSH
    git remote set-url origin git@github.com:errordnk/zed.git
    git remote set-url wrk git@github.com:errordnk/wrk.git
    
    echo ""
    echo "Pushing to wrk..."
    git push wrk main
    echo ""
    echo "Done! Check: https://github.com/errordnk/wrk"
    ;;
    
  2)
    echo ""
    echo "=== Personal Access Token ==="
    echo "1. Go to: https://github.com/settings/tokens/new"
    echo "2. Check 'repo' scope"
    echo "3. Generate token"
    echo "4. Copy the token"
    echo ""
    read -p "Enter your GitHub username: " username
    read -sp "Paste your token: " token
    echo ""
    echo ""
    echo "Pushing to wrk..."
    git push https://$username:$token@github.com/errordnk/wrk.git main
    echo ""
    echo "Done! Check: https://github.com/errordnk/wrk"
    ;;
    
  3)
    echo ""
    cat PUSH_TO_WRK.md
    ;;
    
  4)
    echo "Exiting..."
    exit 0
    ;;
    
  *)
    echo "Invalid choice"
    exit 1
    ;;
esac
