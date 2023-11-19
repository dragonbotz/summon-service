from debian:12.1-slim

RUN apt update && apt install -y openssl

WORKDIR app
COPY target/release/summon-service .

CMD ["./summon-service"]
