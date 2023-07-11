./scripts/build.sh

aws lambda update-function-code \
  --region us-west-1 \
  --function-name sentiment_api \
  --zip-file fileb://bootstrap.zip