#!/usr/bin/bash

# rebuild commands
# docker stop augmented_pathways-app
# docker rm augmented_pathways-app && docker rmi augmented_pathways-img
docker build -t augmented_pathways-img .
docker run -p 0.0.0.0:80:80 --name augmented_pathways-app -d augmented_pathways-img
