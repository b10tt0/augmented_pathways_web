#!/usr/bin/bash

docker build -t augmented-pathways-img .
docker run -d augmented-pathways-img --name augmented-pathways-app  
