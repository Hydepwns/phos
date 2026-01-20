#!/bin/bash
# Update AUR package for new release
set -e

VERSION="${1:-$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)}"
REPO="Hydepwns/phos"
AUR_DIR="${2:-/tmp/aur-phos}"

echo "Updating AUR package to v${VERSION}..."

# Clone AUR repo if needed
if [ ! -d "$AUR_DIR" ]; then
    git clone "ssh://aur@aur.archlinux.org/phos.git" "$AUR_DIR"
fi

cd "$AUR_DIR"
git pull origin master

# Get SHA256 of release tarball
TARBALL_URL="https://github.com/${REPO}/archive/refs/tags/v${VERSION}.tar.gz"
echo "Fetching checksum from ${TARBALL_URL}..."
SHA256=$(curl -sL "$TARBALL_URL" | sha256sum | cut -d' ' -f1)

# Update PKGBUILD
cat > PKGBUILD << EOF
# Maintainer: Hydepwns <hydepwns@proton.me>
pkgname=phos
pkgver=${VERSION}
pkgrel=1
pkgdesc='Universal log colorizer with 99+ program support'
arch=('x86_64' 'aarch64')
url='https://github.com/Hydepwns/phos'
license=('MIT' 'Apache-2.0')
depends=('gcc-libs')
makedepends=('cargo')
source=("\$pkgname-\$pkgver.tar.gz::https://github.com/Hydepwns/\$pkgname/archive/refs/tags/v\$pkgver.tar.gz")
sha256sums=('${SHA256}')

prepare() {
    cd "\$pkgname-\$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "\$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "\$pkgname-\$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release
}

check() {
    cd "\$pkgname-\$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo test --frozen
}

package() {
    cd "\$pkgname-\$pkgver"
    install -Dm755 "target/release/phos" "\$pkgdir/usr/bin/phos"
    install -Dm755 "target/release/phoscat" "\$pkgdir/usr/bin/phoscat"
    install -Dm644 "LICENSE-MIT" "\$pkgdir/usr/share/licenses/\$pkgname/LICENSE-MIT"
    install -Dm644 "LICENSE-APACHE" "\$pkgdir/usr/share/licenses/\$pkgname/LICENSE-APACHE"
    install -Dm644 "README.md" "\$pkgdir/usr/share/doc/\$pkgname/README.md"
}
EOF

# Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

echo ""
echo "PKGBUILD updated:"
grep -E "pkgver|pkgrel|sha256sums" PKGBUILD | head -3

echo ""
echo "To publish:"
echo "  cd $AUR_DIR"
echo "  git add PKGBUILD .SRCINFO"
echo "  git commit -m 'Update to ${VERSION}'"
echo "  git push"
