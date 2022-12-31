function build() {
  cargo build

  # Generate icon.
  /usr/bin/convert -background none -density 1200 -resize 128x128 ./icon.svg ./target/release/icon_128.png

  # Generate completions.
  # Clean.
  rm -rf './target/release/completions/'
  mkdir -p './target/release/completions/'
  # Generate zsh.
  './target/release/bookit' completions --type zsh > './target/release/completions/_bookit.zsh'
}

function test() {
  cargo test
}

function deploy() {
  cargo install --path .
}

function publish() {
  cargo login
  cargo publish
}
