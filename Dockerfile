from python:latest

RUN pip3 install --no-cache-dir tensorflow

COPY target/release/schani_image_recognition /usr/local/bin
COPY tensorflow_models /tensorflow_models

ENV SCRIPT_LOCATION /tensorflow_models/tutorials/image/imagenet
ENV ROCKET_PORT 8000
ENV ROCKET_ADDRESS 0.0.0.0

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/schani_image_recognition"]
