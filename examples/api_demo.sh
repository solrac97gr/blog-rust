#!/bin/bash

# Blog Rust API Demo Script
# Make sure the server is running with: cargo run

echo "🚀 Blog Rust API Demo"
echo "===================="
echo

# Base URL
BASE_URL="http://localhost:8080"

echo "1. Getting all posts..."
curl -s -X GET "$BASE_URL/posts" | jq '.' || curl -s -X GET "$BASE_URL/posts"
echo -e "\n"

echo "2. Creating a new post..."
NEW_POST=$(curl -s -X POST "$BASE_URL/posts" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Demo Post",
    "body": "This is a demo post created by the API demo script!",
    "slug": "demo-post"
  }')

echo "$NEW_POST" | jq '.' 2>/dev/null || echo "$NEW_POST"
POST_ID=$(echo "$NEW_POST" | jq -r '.id' 2>/dev/null || echo "1")
echo -e "\n"

echo "3. Getting the newly created post..."
curl -s -X GET "$BASE_URL/posts/$POST_ID" | jq '.' || curl -s -X GET "$BASE_URL/posts/$POST_ID"
echo -e "\n"

echo "4. Updating the post..."
curl -s -X PUT "$BASE_URL/posts/$POST_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Demo Post",
    "body": "This post has been updated via the API!"
  }' | jq '.' || curl -s -X PUT "$BASE_URL/posts/$POST_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Demo Post",
    "body": "This post has been updated via the API!"
  }'
echo -e "\n"

echo "5. Getting the updated post..."
curl -s -X GET "$BASE_URL/posts/$POST_ID" | jq '.' || curl -s -X GET "$BASE_URL/posts/$POST_ID"
echo -e "\n"

echo "6. Deleting the post..."
curl -s -X DELETE "$BASE_URL/posts/$POST_ID" | jq '.' || curl -s -X DELETE "$BASE_URL/posts/$POST_ID"
echo -e "\n"

echo "7. Trying to get the deleted post (should return 404)..."
curl -s -X GET "$BASE_URL/posts/$POST_ID" | jq '.' || curl -s -X GET "$BASE_URL/posts/$POST_ID"
echo -e "\n"

echo "✅ Demo completed!"
echo "Note: Install 'jq' for better JSON formatting: brew install jq"
