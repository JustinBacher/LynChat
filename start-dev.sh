#!/bin/bash

# Check if .env file exists
if [ ! -f .env ]; then
    echo "Creating .env file..."
    echo "# Hugging Face token for downloading models" > .env
    echo "HF_TOKEN=your_hugging_face_token" >> .env
    echo "" >> .env
    echo "# Other environment variables can be added here" >> .env
    echo ".env file created. Please edit it to add your Hugging Face token."
    echo "You can get a token from https://huggingface.co/settings/tokens"
    exit 1
fi

# Check if HF_TOKEN is set to the default value
if grep -q "HF_TOKEN=your_hugging_face_token" .env; then
    echo "Please edit the .env file to add your Hugging Face token."
    echo "You can get a token from https://huggingface.co/settings/tokens"
    exit 1
fi

# Start Docker Compose
echo "Starting Docker Compose..."
docker-compose up
