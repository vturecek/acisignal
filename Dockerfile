FROM alpine

ADD ./target/x86_64-unknown-linux-musl/release/acisignal ./

CMD ["./acisignal"]
