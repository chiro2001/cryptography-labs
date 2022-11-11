#!/bin/sh

set -x

echo "密码学实验4"
cargo build

key="key"
file="data/lab4-message.txt"
fake="data/lab4-message-fake.txt"
# R_BASE="cargo run -- --binary -k $key"
R_BASE="./target/debug/elgamal --binary -k $key"
# R_BASE="cargo run -- -k $key"
R="$R_BASE -i $file"

rm $key*
sleep 1
echo "生成密钥对：$key $key.pub"
$R -m generate
sleep 1

echo "需要签名的信息：$(cat $file)"
echo "生成签名并验证 $key.sig"
$R -m sign
sleep 1
$R -m check
sleep 1

echo "再次生成签名并验证 $key.sig"
$R -m sign
sleep 1
$R -m check
sleep 1

echo "使用签名对修改后的消息（$(cat $fake)）进行验证："
$R_BASE -m check -i $fake || echo "验证确实没有通过，与预期相符" && exit
echo "验证通过了，与预期不符"