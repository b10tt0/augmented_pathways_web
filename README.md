# Augmented Pathways Web Application

## to build container
`docker build -t augmented-pathways-img .`

## to run
### no reverse proxy, just trunk serve
`docker run -p 0.0.0.0:80:3000 --name augmented-pathways-app -d augmented-pathways-img`
### nginx reverse proxy in webapps docker network
`docker run --network webapps --name augmented-pathways-app -d augmented-pathways-img`

## todo
* change rust web server backend to actix (not trunk) and figure out nginx reverse proxy
