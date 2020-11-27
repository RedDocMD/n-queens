FROM rust:latest

WORKDIR /usr/src/n-queens

COPY Cargo.toml .
COPY src/ src/

RUN cargo build --release

RUN apt update
RUN apt install -y python3-termcolor

COPY test_run.py .

CMD [ "/bin/bash" ]