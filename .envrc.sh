function build() {
  cargo build

  # Generate completions.
  # Clean.
  rm -rf './target/release/completions/'
  mkdir -p './target/release/completions/'
  # Generate zsh.
  './target/release/docket' completions --type zsh > './target/release/completions/_docket.zsh'
}

function test() {
  cargo test
}

function deploy() {
  cargo install --path .
}
