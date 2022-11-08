## 实验三 Hash长度扩展攻击

> 姓名：梁鑫嵘；学号：200110619

### 数据准备阶段

```sh
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
```

