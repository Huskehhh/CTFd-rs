#!/bin/bash

REGISTRY_URL="dagobah:5000"

VERSION=0.0.1

BOT="$REGISTRY_URL/ctf_bot:$VERSION"
REST_API="$REGISTRY_URL/ctf_rest_api:$VERSION"
FRONTEND="$REGISTRY_URL/ctf_frontend:$VERSION"

echo "Building images"
docker build -f bot.Dockerfile -t $BOT .
docker build -f rest-api.Dockerfile -t $REST_API .

pushd frontend
npm run build
docker build -f frontend.Dockerfile -t $FRONTEND
popd

echo "Pushing images to registry"
docker push $BOT
docker push $REST_API
docker push $FRONTEND

echo "Done!"
