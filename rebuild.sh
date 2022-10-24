sudo docker build -t augmented_pathways_web
sudo docker run --network webapps --name ar_web -d augmented_pathways_web
