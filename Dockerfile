FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN cargo install

CMD ["team-ringrostig"]
