#!/usr/bin/bash

# rebuild commands
# docker stop augmented_pathways-app
# docker rm augmented_pathways-app && docker rmi augmented_pathways-img
docker build -t augmented_pathways-img augmented_pathways/.
docker run -p 0.0.0.0:80:3000 --name augmented_pathways-app -d augmented_pathways-img
# systemctl restart nginx
