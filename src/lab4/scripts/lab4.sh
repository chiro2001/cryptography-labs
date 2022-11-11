#!/bin/sh

set -x

echo "密码学实验4"

key="key"
file="data/lab4-message.txt"
fake="data/lab4-message-fake.txt"
# R_BASE="cargo run -- --binary -k $key"
R_BASE="cargo run -- -k $key"
R="$R_BASE -i $file"

rm $key*
sleep 1
echo "生成密钥对：$key $key.pub"
$R -m generate
sleep 1

echo "需要签名的信息：$(cat $file)"
echo "生成签名 $key.sig"
$R -m sign
sleep 1

echo "再次生成签名 $key.sig"
$R -m sign
sleep 1

echo "查看密钥对和生成的签名："
$R -m test
sleep 1

echo "使用签名对信息进行验证："
$R -m check

echo "使用签名对修改后的数据验证："
echo "修改后的数据为 $(cat $fake)"
echo "使用签名对该信息进行验证："
$R_BASE -m check -i $fake || echo "验证确实没有通过，与预期相符" && exit
echo "验证通过了，与预期不符"