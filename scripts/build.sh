cargo lambda build --arm64 --release
mkdir build
cp ./target/lambda/sentiment_api/bootstrap ./build/bootstrap
cd build
zip bootstrap.zip bootstrap
cp bootstrap.zip ..
cd ..
rm -rf build