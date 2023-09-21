#!/bin/bash

function build
{
    docker build -t docker-rust-seed -f docker/prod.Dockerfile .
}

function run
{
    docker run -it --network=host docker-rust-seed
}

"$@"
