FROM rust:latest as builder
ENV APP week5
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/week5 /usr/local/bin/week5
# export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["week5"]