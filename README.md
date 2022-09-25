# cryptography-labs

> 2022 HITSZ 密码学实验

## 运行

1. `make`：得到C/C++的二进制文件
2. `make run`：运行所有 CTests
3. `make submit`：得到提交包
3. `xmake build`：得到 C/C++ 和 Rust 等的所有二进制文件

## Lab1

AES 加密 / 解密。支持 ECB / CBC 模式。

1. `lab1`：完成的实例程序

   ```shell
   $ ./lab1
   ************************$声明信息$****************************
   版权声明：未经授权，禁止传播、使用和用于商业用途
   使用说明：本程序是AES密码演示程序。
   **********************$声明信息$******************************
   ================AES密码算法程序演示================
   
   请输入16个字符的密钥：
   1145141919810aaa
   你输入的密钥为：1145141919810aaa
   请输入你的明文，明文字符长度必须为16的倍数
   1145141919810aaa
   你输入的明文为：1145141919810aaa
   轮密钥..................
   w[0] = 0x31313435 w[1] = 0x31343139 w[2] = 0x31393831 w[3] = 0x30616161 
   w[4] = 0xdfdedb31 w[5] = 0xeeeaea08 w[6] = 0xdfd3d239 w[7] = 0xefb2b358 
   w[8] = 0xeab3b1ee w[9] = 0x4595be6 w[10] = 0xdb8a89df w[11] = 0x34383a87 
   w[12] = 0xe933a6f6 w[13] = 0xed6afd10 w[14] = 0x36e074cf w[15] = 0x2d84e48 
   w[16] = 0x801cf481 w[17] = 0x6d760991 w[18] = 0x5b967d5e w[19] = 0x594e3316 
   w[20] = 0xbfdfb34a w[21] = 0xd2a9badb w[22] = 0x893fc785 w[23] = 0xd071f493 
   w[24] = 0x3c606f3a w[25] = 0xeec9d5e1 w[26] = 0x67f61264 w[27] = 0xb787e6f7 
   w[28] = 0x6bee0793 w[29] = 0x8527d272 w[30] = 0xe2d1c016 w[31] = 0x555626e1 
   w[32] = 0x5a19ff6f w[33] = 0xdf3e2d1d w[34] = 0x3defed0b w[35] = 0x68b9cbea 
   w[36] = 0x1706782a w[37] = 0xc8385537 w[38] = 0xf5d7b83c w[39] = 0x9d6e73d6 
   w[40] = 0xbe898e74 w[41] = 0x76b1db43 w[42] = 0x8366637f w[43] = 0x1e0810a9 
   
   进行AES加密..................
   加密完后的密文的ASCII为：
   0x95 0x14 0x39 0xee 0x6c 0xa4 0x4c 0x1e 0xc2 0xa0 0x46 0xa 0x1a 0xbd 0xdb 0x98 
   请输入你想要写进的文件名，比如'test.txt':
   1145141919810aaa
   已经将密文写进1145141919810aaa中了,可以在运行该程序的当前目录中找到它。
   是否开始解密,1解密，2退出
   1
   [src/lab1/aes.h:683 deAesFile] 当前解密模式：CBC
   请输入要解密的文件名，该文件必须和本程序在同一个目录
   1145141919810aaa
   开始解密.........文件名：1145141919810aaa，密文长度：16
   [src/lab1/aes.h:531 deAes] 当前加密模式：CBC
   解密后的明文ASCII为：
   0x31 0x31 0x34 0x35 0x31 0x34 0x31 0x39 0x31 0x39 0x38 0x31 0x30 0x61 0x61 0x61 
   明文为：1145141919810aaa
   现在可以打开1145141919810aaa来查看解密后的密文了！
   ```

2. `lang-test`：简单测试

3. `lab1-round-test`：覆盖测试

4. `lab1-function-test`：功能测试

5. `lab1-stream-encode`：使用管道/`stdin`/`stdout`的 AES 加密

   ```shell
   $ cat aes-rs | ./lab1-stream-encode 1145141919810aaa CBC > aes-rs-encoded
   ```

6. `lab1-stream-decode`：使用管道/`stdin`/`stdout`的 AES 解密

   ```shell
   $ cat aes-rs-encoded | ./lab1-stream-decode 1145141919810aaa CBC > aes-rs-decoded
   ```

7. `aes-rs`：Rust 实现的 AES 加密 / 解密

   ```shell
   $ ./aes-rs --help
    
   
   USAGE:
       aes-rs [OPTIONS]
   
   OPTIONS:
       -d, --direction <DIRECTION>    Decode or encode data [default: encode] [possible values: decode,
                                      encode, both]
       -h, --help                     Print help information
       -i, --input <INPUT>            Input filename [default: stdin]
       -k, --key <KEY>                Decode / encode key [default: 1145141919810aaa]
       -m, --mode <MODE>              Run mode [default: ECB] [possible values: ECB, CBC]
       -o, --output <OUTPUT>          Output filename [default: stdout]
   ```

