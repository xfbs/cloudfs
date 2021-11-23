FROM rust

# install libfuse, needed for mounting.
RUN apt update && \
    apt install -y libfuse3-dev && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt
