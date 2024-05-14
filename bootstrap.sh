git clone https://github.com/mariotaku/ihslib.git --depth=1

cd ihslib

# replace "protoc-c" with "protoc" in CMakeLists.txt
sed -i 's/protoc-c/protoc/g' CMakeLists.txt

cmake .

make

cd ..