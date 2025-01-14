set -e

cargo +nightly build --release
cp target/release/nrwm .

XEPHYR=$(command -v Xephyr)
xinit ./xinitrc -- \
	"$XEPHYR" \
		:100 \
		-ac \
		-screen 1380x720 \
		-host-cursor

rm -rf nrwm
