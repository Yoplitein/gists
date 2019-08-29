# Maintainer: Sanpi <sanpi+aur@homecomputing.fr>
pkgname=emsdk
pkgver=1.35
pkgrel=6
pkgdesc='The Emscripten SDK'
arch=('x86_64')
url='https://kripken.github.io/emscripten-site/'
license=('MIT')
depends=('python2' 'cmake')
source=("https://github.com/emscripten-core/emsdk/archive/master.zip"
        'emsdk.sh')
sha256sums=('SKIP'
            'SKIP')

package()
{
    install --mode 755 --directory "$pkgdir/usr/bin"
    install --mode 755 emsdk.sh "$pkgdir/usr/bin/emsdk"

    cd "$srcdir/${pkgname}-master"

    install --mode 755 --directory "$pkgdir/usr/lib/$pkgname"
    install --mode 755 emsdk.py "$pkgdir/usr/lib/$pkgname"
    install --mode 755 emsdk_env.sh "$pkgdir/usr/lib/$pkgname"
    install --mode 644 emsdk_manifest.json "$pkgdir/usr/lib/$pkgname"
    install --mode 644 emscripten-releases-tags.txt "$pkgdir/usr/lib/$pkgname"
    install --mode 644 legacy-binaryen-tags.txt "$pkgdir/usr/lib/$pkgname"
    install --mode 644 legacy-emscripten-tags.txt "$pkgdir/usr/lib/$pkgname"
}
