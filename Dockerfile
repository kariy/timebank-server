FROM rust:1-alpine3.16

COPY . .

RUN rust build --release

CMD [ "./target/debug/server" ]