#!/bin/bash
# 获取ipv6
ipv6=`ifconfig | grep inet6 | grep 2409 | grep " 64" | awk '{print $2}' | head -1`
echo "ipv6: $ipv6"

# 对比上次的ipv6并更新
file="ipv6"

if [ -e "$file" ]; then
    # 文件存在，比较内容
    if [ "$(cat "$file")" != "$ipv6" ]; then
        # 内容不一样，将content写入文件
        echo "$ipv6" > "$file"
        echo "内容已更新。"
    else
        echo "内容相同，无需更新。"
        exit 0
    fi
else
    # 文件不存在，创建文件并写入content
    echo "$ipv6" > "$file"
    echo "文件已创建并内容已写入。"
fi
# 加密ipv6
rsafile="rsa_ipv6"
base64_rsafile="base64_rsa_ipv6"
#-pubin指明输入的是公钥文件，因为默认为私钥文件，所以这里需要指明
openssl rsautl -encrypt -in $file -inkey key/PKCS8_publicKey.pem -pubin -out $rsafile
base64 $rsafile > $base64_rsafile
# 上报ipv6
rsa_ipv6=`cat $base64_rsafile | awk '{printf "%s", $0}'`
name=$1
result=`curl --header "Content-Type: application/json" --request POST  --data '{"name": "'"$name"'", "ipv6": "'"$rsa_ipv6"'"}' "ddns.manxiaozhi.com/ipv6"`

echo "完成上报 $result $rsa_ipv6"

remote_ipv6=`curl $2/ipv6?name=$name`

echo "读取 $remote_ipv6"