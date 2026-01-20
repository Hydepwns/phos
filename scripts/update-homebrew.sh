#!/bin/bash
# Update Homebrew formula for new release
set -e

VERSION="${1:-$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)}"
REPO="Hydepwns/phos"
TAP_DIR="${2:-/tmp/homebrew-phos}"

echo "Updating Homebrew formula to v${VERSION}..."

# Clone tap if needed
if [ ! -d "$TAP_DIR" ]; then
    git clone "https://github.com/Hydepwns/homebrew-phos.git" "$TAP_DIR"
fi

cd "$TAP_DIR"
git pull origin main

# Get SHA256 of release tarball
SHA256=$(curl -sL "https://github.com/${REPO}/archive/refs/tags/v${VERSION}.tar.gz" | shasum -a 256 | cut -d' ' -f1)

# Update formula
sed -i '' "s|/tags/v[0-9.]*\.tar\.gz|/tags/v${VERSION}.tar.gz|" phos.rb
sed -i '' "s/sha256 \"[a-f0-9]*\"/sha256 \"${SHA256}\"/" phos.rb

# Get program count and update
COUNT=$(cd - >/dev/null && cargo run --bin phos --quiet -- list 2>/dev/null | grep -c "^\s\s" || echo "99")
sed -i '' "s/[0-9]* programs/${COUNT} programs/" phos.rb

echo "Formula updated:"
grep -E "url|sha256|desc" phos.rb | head -3

echo ""
echo "To publish: cd $TAP_DIR && git add phos.rb && git commit -m 'phos ${VERSION}' && git push"
