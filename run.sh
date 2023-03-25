#!/bin/bash

function build_sbf() {
    RUST_LOG=info cargo build-sbf --manifest-path=program/Cargo.toml
}

case $1 in
    "build-sbf")
	build_sbf
	;;
    "deploy")
	build_sbf
	solana program deploy --use-quic -k test.json -u localhost program/target/deploy/helloworld.so
	;;
    "client")
	cargo run --manifest-path=client/Cargo.toml -- -C config.yml -k program/target/deploy/helloworld-keypair.json -u localhost
	;;
    "clean")
	git clean -fdx
	;;
    *)
	echo "usage: $0 build-sbf"
	;;
esac
