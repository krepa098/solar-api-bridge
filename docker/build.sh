#!/bin/bash

if docker build -t solar-api-bridge . ; then
    IMAGE_FILE=solar-api-bridge.tgz
    docker save solar-api-bridge | gzip > $IMAGE_FILE

    # restore with: gunzip -c solar-api-bridge.tgz | docker load
else
    echo "docker build failed!"
fi