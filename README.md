# Schani image recognition
Microservice that performs image recognition

[![Build Status](https://travis-ci.org/schani-rs/schani_image_recognition.svg?branch=master)](https://travis-ci.org/schani-rs/schani_image_recognition)

## API usage

You can classify an image by starting the webservice and sending a request with

```bash
curl -X POST localhost:8000/recognize --data-binary @resources/sample1.jpg
```
