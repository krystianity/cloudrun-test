FROM scratch
ADD target/x86_64-unknown-linux-musl/release/cloudrun_test /
ENV RUST_LOG info
CMD ["/cloudrun_test"]