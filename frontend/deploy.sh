#!/bin/bash

echo "Building..."
npm run build

echo "Packing build ready for deployment"
tar -cvf ./deploy.tar --exclude='*.map' ./captain-definition ./build/*

echo "Deploying..."
caprover deploy -t ./deploy.tar

echo "Done! Cleaning up tar"
rm -rf deploy.tar
