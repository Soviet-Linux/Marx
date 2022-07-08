FILE=$HOME/.cargo/env
if test -f "$FILE"; then
    git clone https://github.com/TheHolyTachanka/Duck-CLI
    cd Duck-CLI
    build
    install
    cd ..
    rm -rf Duck-CLI
else
	curl https://sh.rustup.rs -sSf | sh -s -- --profile normal --default-toolchain stable
fi



build () {
    echo '############'
    echo '##Building##'
    echo '############'
    cargo build --release
}

install () {
    sudo mv target/release/Duck-CLI /usr/bin/
    sudo chmod +x /usr/bin/Duck-CLI
    echo '##############'
    echo '##installed##'
    echo '##############'
}