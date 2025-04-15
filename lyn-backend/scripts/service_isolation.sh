#!/bin/bash

# Set up test environment
echo "Setting up test environment..."
docker-compose up -d

# Wait for services to be ready
echo "Waiting for services to be ready..."
sleep 5

# Test Web Service to User Data Service isolation
echo "Testing Web Service to User Data Service isolation..."
curl -X POST http://localhost:8080/api/auth/login -H "Content-Type: application/json" -d '{"username":"testuser","password":"testpass"}' >/tmp/login_response.json
TOKEN=$(jq -r '.token' /tmp/login_response.json)

# Test Web Service to LLM Proxy Service isolation
echo "Testing Web Service to LLM Proxy Service isolation..."
curl -X POST http://localhost:8080/api/chat -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN" -d '{"message":"Hello, world!","provider":"openai","model_name":"gpt-3.5-turbo"}' >/tmp/chat_response.json

# Check if the LLM Proxy Service sanitized the message
echo "Checking if the LLM Proxy Service sanitized the message..."
curl -X POST http://localhost:8082/api/llm/sanitize -H "Content-Type: application/json" -d '{"message":"Hello, my email is test@example.com and my phone number is 555-555-5555"}' >/tmp/sanitize_response.json
DETECTED_PII=$(jq '.detected_pii | length' /tmp/sanitize_response.json)
if [ "$DETECTED_PII" -gt 0 ]; then
	echo "Sanitization test passed: PII detected"
else
	echo "Sanitization test failed: No PII detected"
	exit 1
fi

# Test User Data Service to LLM Proxy Service isolation
echo "Testing User Data Service to LLM Proxy Service isolation..."
# TODO: Implement this test

# Clean up
echo "Cleaning up..."
docker-compose down

echo "All tests passed!"
