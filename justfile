serve: 
  trunk serve -w src

build-web:
  rm -rf ./docs
  trunk build --release --features=final --dist docs --minify --public-url ./
