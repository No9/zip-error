# zip-error
A sample project to demonstrate an issue with Rust Zip

```
base64 /dev/urandom | head -c 512M >./test.file
cat ./test.file | ./target/debug/zip-error
unzip test.zip
diff new_test.file test.file
```
