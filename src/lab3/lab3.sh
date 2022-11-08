#!/bin/zsh

echo "Preparing..."
if [ ! -f Labsetup.zip ]; then
# 下载实验包
curl "https://gitee.com/hitsz-cslab/cryptography-labs/raw/master/stupkt/lab3/Labsetup.zip" -O Labsetup.zip
fi
# 下载 compute_padding.py
if [ ! -f compute_padding.py ]; then
curl "https://gitee.com/hitsz-cslab/cryptography-labs/raw/master/stupkt/lab3/code-%E4%B8%8D%E8%83%BD%E6%89%A9%E6%95%A3%E5%88%B0%E7%BD%91%E4%B8%8A/compute_padding.py" -O compute_padding.py
fi
# 给 compute_padding.py 打个 patch，让它能够接收命令行参数
sed -i 's#^cmd = "myname=SEEDManual\&uid=1001\&lstcmd=1"#import sys; cmd = "myname=SEEDManual\&uid=1001\&lstcmd=1" if len(sys.argv) < 2 else sys.argv[1]#g' compute_padding.py
# 下载 url_length_extension.c
if [ ! -f url_length_extension.c ]; then
curl "https://gitee.com/hitsz-cslab/cryptography-labs/raw/master/stupkt/lab3/code-%E4%B8%8D%E8%83%BD%E6%89%A9%E6%95%A3%E5%88%B0%E7%BD%91%E4%B8%8A/url_length_extension.c" -O url_length_extension.c
fi
# 解压实验包
if [ ! -d Labsetup ]; then
unzip -o Labsetup.zip
fi
echo "Prepare done"

function fetch() {
  printf "Fetching url %s\n" $1
  curl -s "$1" | ./totext.py
}

# host=www.seedlab-hashlen.com
host=localhost
port=80
port=8000
prefix="http://${host}:${port}/?"
name=QichenWAN

echo "Starting flask environmemnt..."
FLASK_APP=./Labsetup/image_flask/app/www flask run --host ${host} --port ${port} & pid=$!
sleep 1
# echo "FLASK_APP=./Labsetup/image_flask/app/www flask run --host ${host} --port ${port}"
# read
echo "Environment startted to ${prefix}, pid = ${pid}"

echo "#Test"
fetch "${prefix}myname=JohnDoe&uid=1001&lstcmd=1&mac=7d5f750f8b3203bd963d75217c980d139df5d0e50d19d6dfdb8a7de1f8520ce3"

function hash_payload() {
  message=$(urldecode "$2")
  # message="$2"
  echo -n "$1:${message}" | sha256sum | awk '{print $1}'
}

function hashme() {
  cmd="$1"
  payload="myname=${name}&uid=1001&lstcmd=1${cmd}"
  echo -n "${payload}" $(hash_payload "123456" "${payload}")
}

function urldecode() {
  # printf $(echo -n "$1" | sed 's/\(%\)\([0-9a-fA-F][0-9a-fA-F]\)/\\x\2/g')
  printf $(echo -n "$1" | python3 urldecode.py)
}

echo "# task 1"
cmd="&download=secret.txt"
res=$(hashme ${cmd})
payload=$(echo ${res} | awk '{print $1}')
mac=$(echo ${res} | awk '{print $2}')
echo "mac = ${mac}, payload = ${payload}, cmd = ${cmd}"
fetch "${prefix}${payload}&mac=${mac}"

echo "# task 2"
kfile="./Labsetup/image_flask/app/LabHome/key.txt"
while read line
do
  args=$(echo ${line} | sed 's/:/ /g' | awk '{print "myname=" $2 "&uid=" $1 "&lstcmd=1"}')
	paddings=$(python3 compute_padding.py ${args})
  echo "${line} padding to: ${paddings}"
done < ${kfile}

echo "# task 3"
cmd="&download=secret.txt"
key_text=$(cat LabHome/key.txt)
uid="${key_text:0:4}"
key="${key_text:5:12}"
echo "uid is ${uid}, key is ${key}"
payload="myname=${name}&uid=${uid}&lstcmd=1"
paddings=$(python3 compute_padding.py ${payload})
paddings_decoded=$(urldecode "${paddings}")
echo "paddings: ${paddings}, decoded: ${paddings_decoded}"
payload_ext=$(echo "${paddings}" | sed "s/AAAAAA\://g")
# payload_ext="${payload}%32%20%00%32%32"
echo "hash: ${key}:${payload_ext}"

# mac0=$(hash_payload "${key}" "${payload_ext}")
# mac0=$(echo -n "${key}:${payload_ext}" | python3 urldecode.py | sha256sum | awk '{print $1}')
mac0=$(echo -n "****************************************************************" | sha256sum | awk '{print $1}')
# mac0="4ac5b1038ff9a79b5ed7a8cbbb97f2b658871d6a02c0d12e1aec4bba51ec19d9"
echo "original mac is ${mac0} hash("${key}":"${payload_ext}")"
i=1
# i=8
mkdir -p tests
proc_file="tests/url_length_extension_modifiled.c"
cp url_length_extension.c ${proc_file}
oiginal_words=(0x3912fe08 0x949c7c09 0xbd2825b0 0x1a2e8e9c 0x151d84be 0x0106e858 0x4e9006b8 0x8a22555f)
# mac is 64x4 bit, split to 8x uint32_t
while ((${#mac0} > 0))
do
  word=${mac0:0:8}
  echo "word ${oiginal_words[i]} (index ${i}) => word 0x${word} | ${mac0}"
  sed -i "s/${oiginal_words[i]}/0x${word}/g" "${proc_file}"
  # sed -i "s/${oiginal_words[i]}/0x0/g" "${proc_file}"
  declare -i i=${i}+1
  # declare -i i=${i}-1
  mac0=${mac0:8:64}
done
rm p3
gcc ${proc_file} -lcrypto -o p3 -Wno-deprecated-declarations
mac=$(./p3)
echo "new mac is ${mac}"
mac_check=$(echo -n "${key}:${payload_ext}${cmd}" | python3 urldecode.py | sha256sum | awk '{print $1}')
# mac_check=$(hash_payload "${key}" "${payload_ext}${cmd}")
# echo -n "$(urldecode "%32%00%32")" > message
# echo -n "\x32\x00\x32" > message
echo "mac should be ${mac_check} hash("${key}":"${payload_ext}${cmd}")"
if [ "${mac_check}" != "${mac}" ]; then
  echo "Failed!"
  kill ${pid}
  return
fi
result=$(fetch "${prefix}${payload_ext}${cmd}&mac=${mac}")
echo "$result"
failed_msg="Sorry"
if [[ "${result}" =~ "${failed_msg}" ]]; then
  echo "Failed!"
  # read
else
  echo "Can visit results now, enter to leave"
  read
fi
kill ${pid}