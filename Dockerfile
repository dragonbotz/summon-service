from debian:12.1-slim

WORKDIR app
COPY target/release/summon-service .

CMD ["./summon-service"]
