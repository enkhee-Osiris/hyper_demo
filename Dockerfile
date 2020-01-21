FROM rust:slim

WORKDIR /usr/src/app

COPY Cargo.* ./

RUN mkdir -p ./src && echo -n "fn main() {\n    println!(\"Hello World);\n}" > ./src/main.rs && cargo fetch

COPY . .

EXPOSE 3001

CMD ["cargo", "run"]
