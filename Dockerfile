FROM scratch

ADD Rocket.toml /
ADD target/x86_64-unknown-linux-musl/release/rtm /
EXPOSE 8000

CMD ["/rtm"]