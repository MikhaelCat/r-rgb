# Maintainer: MikhaelCat <mikhaelcat@users.noreply.github.com>
pkgname=rrgb-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="High-performance Rust RGB controller for MSI SteelSeries laptops (KLC keyboard + ALC lightbar/logo)"
arch=('x86_64')
url="https://github.com/MikhaelCat/r-steelseries"
license=('MIT' 'Apache-2.0')
depends=('glibc')
makedepends=('cargo')
provides=('rrgb')
conflicts=('rrgb')
source=("git+https://github.com/MikhaelCat/r-steelseries.git")
sha256sums=('SKIP')

build() {
  cd "$srcdir/r-steelseries"
  cargo build --release --locked
}

check() {
  cd "$srcdir/r-steelseries"
  cargo test --release --locked
}

package() {
  cd "$srcdir/r-steelseries"

  # Binary
  install -Dm755 "target/release/rrgb" "$pkgdir/usr/bin/rrgb"

  # udev rules
  install -Dm644 "packaging/50-rrgb.rules" "$pkgdir/usr/lib/udev/rules.d/50-rrgb.rules"

  # systemd services
  install -Dm644 "packaging/rrgb.service" "$pkgdir/usr/lib/systemd/user/rrgb.service"
  install -Dm644 "packaging/rrgb-restore.service" "$pkgdir/usr/lib/systemd/system/rrgb-restore.service"

  # Licenses
  install -Dm644 "LICENSE-MIT" "$pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT"
  install -Dm644 "LICENSE-APACHE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE-APACHE"

  # Documentation
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}
