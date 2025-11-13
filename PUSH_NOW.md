# Push to GitHub - Quick Guide

**Status:** 15 commits ready to push to https://github.com/errordnk/wrk.git

## ⚡ Quick Push (30 seconds)

### Step 1: Get Token
Open: https://github.com/settings/tokens/new

- **Note**: `Claude Code`
- **Scopes**: ✅ Check **`repo`** only
- Click "Generate token"
- **COPY the token** (shows once!)

### Step 2: Push
Replace `YOUR_TOKEN` with your token:

```bash
git push https://errordnk:YOUR_TOKEN@github.com/errordnk/wrk.git main
```

Example:
```bash
git push https://errordnk:ghp_1234567890abcdef@github.com/errordnk/wrk.git main
```

## ✅ After Push

1. Verify: https://github.com/errordnk/wrk
2. Clone on Windows: `git clone https://github.com/errordnk/wrk.git`
3. Build: See WINDOWS_BUILD.md

## 🔧 Alternative: Store Credentials

If you want to avoid typing token every time:

```bash
# Configure git
git config --global user.name "errordnk"
git config --global user.email "your_email@example.com"
git config --global credential.helper store

# Push (will save credentials)
git push https://errordnk:YOUR_TOKEN@github.com/errordnk/wrk.git main

# Next time just:
git push wrk main
```

## 📦 What Will Be Pushed

Complete Zed Terminal with:
- Windows Terminal style UI (tabs in title bar)
- Terminal connection profiles (SSH, local)
- AI integration (Zed Free, no API keys needed)
- Complete Windows build guide
- All documentation and examples
- 15 commits of improvements

Repository: https://github.com/errordnk/wrk
