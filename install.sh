build () {
    echo '############'
    echo '##Building##'
    echo '############'
    cargo build --release
}

install () {
    sudo mv target/release/marx /usr/bin/
    sudo chmod +x /usr/bin/marx
    echo '##############'
    echo '##installed##'
    echo '##############'
}

FILE=$HOME/.cargo/env
if test -f "$FILE"; then
    git clone https://github.com/TheHolyTachanka/Marx
    cd Marx
    build
    install
    cd ..
    rm -rf Marx
else
    curl https://sh.rustup.rs -sSf | sh -s -- --profile normal --default-toolchain stable
fi
