#生成私钥，默认生成的是PKCS1格式的
openssl genrsa -out privateKey.pem 2048
#如果需要生成PKCS8格式，需要进行转化
openssl pkcs8 -topk8 -in privateKey.pem -out PKCS8_privateKey.pem -nocrypt
#生成对应的公钥
openssl rsa -in PKCS8_privateKey.pem -out PKCS8_publicKey.pem -pubout