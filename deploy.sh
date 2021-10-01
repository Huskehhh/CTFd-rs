#!/bin/bash

REGISTRY_URL="registry.husk.pro"

VERSION=0.1.2

BOT="$REGISTRY_URL/ctf_bot:$VERSION"
REST_API="$REGISTRY_URL/ctf_rest_api:$VERSION"
FRONTEND="$REGISTRY_URL/ctf_frontend:$VERSION"

PLATFORM=linux/arm64/v8

echo "Building images"
docker buildx build -f bot.Dockerfile -t $BOT --platform $PLATFORM .
docker buildx build -f rest-api.Dockerfile -t $REST_API --platform $PLATFORM .

pushd frontend
docker buildx build -f frontend.Dockerfile -t $FRONTEND --platform $PLATFORM .
popd

echo "Pushing images to registry"
docker push $BOT
docker push $REST_API
docker push $FRONTEND

echo "Done!"
