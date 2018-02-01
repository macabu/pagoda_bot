FROM rust:1.23.0
WORKDIR /usr/src/pagoda_bot
COPY . .
RUN cargo install
CMD ["pagoda_bot"]