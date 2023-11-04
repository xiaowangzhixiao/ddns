# 读取
name=$1
remote_ipv6=`curl ddns.manxiaozhi.com/ipv6?name=$name`
echo "读取 $remote_ipv6"
# 解密
rsafile="get_rsa_ipv6"
outfile="get_ipv6"
echo $remote_ipv6 | base64 -d -o $rsafile
#$rsafile指需要解密的密文，$outfile指解密后的数据文件
openssl rsautl -decrypt -in $rsafile -inkey ../key/PKCS8_privateKey.pem -out $outfile

cat $outfile
