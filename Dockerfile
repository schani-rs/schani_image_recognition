from debian:latest

RUN apt-get update
RUN apt-get install -y \
       python3.5 \
       --no-install-recommends

COPY target/release/schani_image_recognition /usr/local/bin

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/schani_image_recognition"]
