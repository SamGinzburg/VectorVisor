openssl genrsa -out key.pem 2048
openssl rsa -in key.pem -pubout > key.pub
openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in key.pem -out pkcs8.key
