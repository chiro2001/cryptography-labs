# cryptography-labs

> 2022 HITSZ 密码学实验

## 运行

1. `make`：得到C/C++的二进制文件
2. `make run`：运行所有 CTests
3. `make submit`：得到提交包
3. `xmake build`：得到 C/C++ 和 Rust 等的所有二进制文件
3. `make docs`：通过 `texlive` 构建提交文档

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

## Lab2

RSA 密钥生成 / 加密 / 解密 / 密钥测试。

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ ./rsa-linux-x86_64 -h
Usage: rsa-linux-x86_64 [OPTIONS]

Options:
  -m, --mode <MODE>            Run mode [default: generate] [possible values: generate, encode, decode, test]
  -k, --key <KEY>              Key path, generate/detect `path' and `path.pub' [default: key]
  -c, --comment <COMMENT>      Attach comment to key files [default: "RSA-RS COMMENT"]
      --binary                 Output key in base64 format
  -i, --input <INPUT>          Input filename [default: stdin]
  -o, --output <OUTPUT>        Output filename [default: stdout]
      --prime-min <PRIME_MIN>  Min prime bits [default: 14]
      --prime-max <PRIME_MAX>  Max prime bits [default: 512]
  -r, --rounds <ROUNDS>        Miller Rabin calculate rounds [default: 10]
      --time-max <TIME_MAX>    Max time in mill seconds that trying to generate a prime [default: 1000]
  -s, --silent                 Disable log output
      --retry                  Retry when failed to generate primes
  -t, --threads <THREADS>      Calculate in <THREADS> threads [default: 20]
  -h, --help                   Print help information
```

密钥生成

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ ./rsa-linux-x86_64 -m generate -k key 
Run args: RSA { mode: "generate", key: "key", comment: "RSA-RS COMMENT", binary: false, input: "stdin", output: "stdout", prime_min: 14, prime_max: 512, rounds: 10, time_max: 1000, silent: false, retry: true, threads: 20 }
Done generation in 15 tries after 563 ms
Failed generation in 45 tries after 1009 ms
Failed generation in 60 tries after 1026 ms
Failed generation in 45 tries after 1043 ms
Failed generation in 45 tries after 1049 ms
Failed generation in 45 tries after 1060 ms
Failed generation in 45 tries after 1063 ms
Failed generation in 45 tries after 1089 ms
Failed generation in 45 tries after 1089 ms
Failed generation in 60 tries after 1115 ms
Failed generation in 60 tries after 1126 ms
Failed generation in 45 tries after 1131 ms
Failed generation in 60 tries after 1143 ms
Failed generation in 60 tries after 1154 ms
Failed generation in 60 tries after 1157 ms
Failed generation in 60 tries after 1162 ms
Failed generation in 60 tries after 1164 ms
Done generation in 60 tries after 1165 ms
Done generation in 45 tries after 1166 ms
Failed generation in 60 tries after 1165 ms
Use cached prime: 11630650943675141937779600860164392907676984206916657617564549735086824304628002597593635872207376435359233505759474625776767029128728249413081464681494217
Use cached prime: 4686533752114096249029701209240726249533547904621370626209098435830353368768746089519914911412944837588925493817728625762795150992683258443277687104252423
(d * e) % f = 150330413373214006343081365639886829019519385526364571891272230577213903583096431432090964548990111193436205085256673695289188702621152623459995803452360631219093050865131248602013968612982467439878186453697950079216196984828818759401754433700243345055505777596845822252142590147448271591644172449179689127600925515776646473065739154780148620217569506692982096568063549228828918079015585346210817507982502445489470608236127952788745578895323505915013254961608705 % 97870731250985564623015522731986540041321887839942203016456390534448411119245216192921820674936003494282845822085646667019623943737663390816986843733314207569111593649135075338002245913975266063274072078379252204335339037096226378796144150370833097121247629502565984144548912385744370079712402829821621072256 = 1
get keys: KeySet { public: Key { base: 4686533752114096249029701209240726249533547904621370626209098435830353368768746089519914911412944837588925493817728625762795150992683258443277687104252423, m: 97870731250985564623015522731986540041321887839942203016456390534448411119245216192921820674936003494282845822085646667019623943737663390816986843733314227614659667756051735166349200814935133128406002982242439800830852706418954744760701868903543396835986551217439767559341046460767657218440816102753468279289 }, private: Key { base: 32077100331433637675744493842254019082711536768074859315168843376879817656734293876192590687856533019905084175635399999667007287688037521925447051783124526551778704694277885262000009822699828695741281539651603600818279121713560185006227597556114598148997667757217472312796108644362178353646142214156271889335, m: 97870731250985564623015522731986540041321887839942203016456390534448411119245216192921820674936003494282845822085646667019623943737663390816986843733314227614659667756051735166349200814935133128406002982242439800830852706418954744760701868903543396835986551217439767559341046460767657218440816102753468279289 } }
get key_pair: KeyPair { public: KeyData { mode: "PUBLIC_", comment: "RSA-RS COMMENT", key: Key { base: 4686533752114096249029701209240726249533547904621370626209098435830353368768746089519914911412944837588925493817728625762795150992683258443277687104252423, m: 97870731250985564623015522731986540041321887839942203016456390534448411119245216192921820674936003494282845822085646667019623943737663390816986843733314227614659667756051735166349200814935133128406002982242439800830852706418954744760701868903543396835986551217439767559341046460767657218440816102753468279289 }, header: "-----BEGIN RSA-512 PUBLIC_ KEY-----", footer: "-----END RSA-512 PUBLIC_ KEY-----" }, private: KeyData { mode: "PRIVATE", comment: "RSA-RS COMMENT", key: Key { base: 32077100331433637675744493842254019082711536768074859315168843376879817656734293876192590687856533019905084175635399999667007287688037521925447051783124526551778704694277885262000009822699828695741281539651603600818279121713560185006227597556114598148997667757217472312796108644362178353646142214156271889335, m: 97870731250985564623015522731986540041321887839942203016456390534448411119245216192921820674936003494282845822085646667019623943737663390816986843733314227614659667756051735166349200814935133128406002982242439800830852706418954744760701868903543396835986551217439767559341046460767657218440816102753468279289 }, header: "-----BEGIN RSA-512 PRIVATE KEY-----", footer: "-----END RSA-512 PRIVATE KEY-----" } }
Generated key files: key, key.pub
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ cat key.pub 
-----BEGIN RSA-512 PUBLIC_ KEY-----
QAAAAIAAAAAHqlxsdbu3bxnEjY1nQyVsEh7G9PC2nJ41walPVXmQYIL31MnM362l6WGXkP
xLClBOSsSuZ1Q+0c1WnQTATHtZ+enR8USft8AVsv15ZgGwD+mDLfFBd9XU01Dg2eDFJs8f
ihTjbYr6We4HTMa0AhAfWcmxIOAssZTov8rDpYc3eG3ZyfmCGblYzszLLe/Rj00D7kAnoD
+ROcK53V3w1/1chgSOGd4zOmNCBd5uZqnXDX6xWKVTj2Wi9DM6f0dfX4tQVUJMSUNfUlNB
LVJTIENPTU1F
-----END RSA-512 PUBLIC_ KEY-----
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ cat key    
-----BEGIN RSA-512 PRIVATE KEY-----
gAAAAIAAAAC3K5bt0KQeGwQu+HxEeczTCEtsuGQwBLURSy0/fjPSf8rSglPfSk7Vtyv7lo
bIx4eYk6WSmzsP9OYCdDCO68wDSXzctDA1ojBdIGqZKxvnjEiV6iga8wqBi9ZaMf+AY5tW
9OI3EKMpjvzcqZwrs0ld2vTPvVdihGUnnpCYEuetLfnp0fFEn7fAFbL9eWYBsA/pgy3xQX
fV1NNQ4NngxSbPH4oU422K+lnuB0zGtAIQH1nJsSDgLLGU6L/Kw6WHN3ht2cn5ghm5WM7M
yy3v0Y9NA+5AJ6A/kTnCud1d8Nf9XIYEjhneMzpjQgXebmap1w1+sVilU49lovQzOn9HX1
+LUFJJVkFURVJTQS1SUyBDT01NRU5U
-----END RSA-512 PRIVATE KEY-----
```

加密

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ ./rsa-linux-x86_64 -i aes-rs-windows-x86_64.exe -o tmp -m encode -k key
Run args: RSA { mode: "encode", key: "key", comment: "RSA-RS COMMENT", binary: false, input: "aes-rs-windows-x86_64.exe", output: "tmp", prime_min: 14, prime_max: 512, rounds: 10, time_max: 1000, silent: false, retry: true, threads: 20 }
group size 64, input => output: 64 => 128
source chunk: 13040
  [00:00:25] [###############################] 815.00 KiB/815.00 KiB (0s)
read filesize: 834560, data filesize: 834560 res chunk: 13040
Done
```

解密

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ ./rsa-linux-x86_64 -o aes.exe -i tmp -m decode -k key
Run args: RSA { mode: "decode", key: "key", comment: "RSA-RS COMMENT", binary: false, input: "tmp", output: "aes.exe", prime_min: 14, prime_max: 512, rounds: 10, time_max: 1000, silent: false, retry: true, threads: 20 }
group size 128, input => output: 128 => 64
source chunk: 13040
  [00:00:48] [################################] 1.59 MiB/1.59 MiB (0s)
read filesize: 1669120, data filesize: 834560 res chunk: 13040
Done
```

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ diff aes.exe aes-rs-windows-x86_64.exe 
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ 
```

密钥测试

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release git:(master) ✗ ./rsa-linux-x86_64 -m test -k key                    
Run args: RSA { mode: "test", key: "key", comment: "RSA-RS COMMENT", binary: false, input: "stdin", output: "stdout", prime_min: 14, prime_max: 512, rounds: 10, time_max: 1000, silent: false, retry: true, threads: 20 }
PUBLIC_ key, comment: RSA-RS COMME
PRIVATE key, comment: RSA-RS COMMENT
start testing key pair
get key_pair: KeyPair { public: KeyData { mode: "PUBLIC_", comment: "RSA-RS COMME", key: Key { base: 4686533752114096249029701209240726249533547904621370626209098435830353368768746089519914911412944837588925493817728625762795150992683258443277687104252423, m: 97870731250985564623015522731986540041321887839942203016456390534448411119245216192921820674936003494282845822085646667019623943737663390816986843733314227614659667756051735166349200814935133128406002982242439800830852706418954744760701868903543396835986551217439767559341046460767657218440816102753468279289 }, header: "-----BEGIN RSA-512 PUBLIC_ KEY-----", footer: "-----END RSA-512 PUBLIC_ KEY-----" }, private: KeyData { mode: "PRIVATE", comment: "RSA-RS COMMENT", key: Key { base: 32077100331433637675744493842254019082711536768074859315168843376879817656734293876192590687856533019905084175635399999667007287688037521925447051783124526551778704694277885262000009822699828695741281539651603600818279121713560185006227597556114598148997667757217472312796108644362178353646142214156271889335, m: 97870731250985564623015522731986540041321887839942203016456390534448411119245216192921820674936003494282845822085646667019623943737663390816986843733314227614659667756051735166349200814935133128406002982242439800830852706418954744760701868903543396835986551217439767559341046460767657218440816102753468279289 }, header: "-----BEGIN RSA-512 PRIVATE KEY-----", footer: "-----END RSA-512 PRIVATE KEY-----" } }
  [00:00:05] [###################################] 62.50 KiB/62.50 KiB (0s)
Test pass
```

## Lab4

ElGamal 生成密钥 / 数字签名 / 验证签名

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ ./elgamal -h
Usage: elgamal [OPTIONS]

Options:
  -m, --mode <MODE>            Run mode [default: generate]
  -k, --key <KEY>              Key path, generate/detect `path' and `path.pub' [default: key]
  -c, --comment <COMMENT>      Attach comment to key files [default: "ELGAMAL-RS COMMENT"]
      --binary                 Output key in binary format
  -i, --input <INPUT>          Input filename [default: stdin]
  -o, --output <OUTPUT>        Output filename [default: stdout]
      --prime-min <PRIME_MIN>  Min prime bits [default: 10]
      --prime-max <PRIME_MAX>  Max prime bits [default: 12]
  -r, --rounds <ROUNDS>        Miller Rabin calculate rounds [default: 10]
      --time-max <TIME_MAX>    Max time in mill seconds that trying to generate a prime [default: 1000]
  -s, --silent                 Disable log output
      --retry                  Retry when failed to generate primes
  -t, --threads <THREADS>      Calculate in <THREADS> threads [default: 5]
  -h, --help                   Print help information
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ 
```

密钥生成

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ ./elgamal -m generate -k key
Done generation in 15 tries after 0 ms
Done generation in 15 tries after 0 ms
Done generation in 30 tries after 0 ms
Done generation in 15 tries after 0 ms
Done generation in 15 tries after 0 ms
Use cached prime: 1999
Use cached prime: 3359
Use cached prime: 1693
generated key set: ElGamalKey { public: ElGamalPublicKey { p: 3359, g: 1693, y: 884 }, private: ElGamalPrivateKey { x: 2517 } }
Save key to file key key.pub
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ 
```

文件签名

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ ./elgamal -k key -m sign -i elgamal 
key: ElGamalKey { public: ElGamalPublicKey { p: 3359, g: 1693, y: 884 }, private: ElGamalPrivateKey { x: 2517 } }
k: 499, sign: ElGamalSign { r: 280, s: 420 }
Save sign to file key.sig
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ 
```

验证签名

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ ./elgamal -k key -m check -i elgamal
884^280 * 280^420 mod 3359 =?= 1693^53542441927936905103363006904913156046451446391687346369727530240800526517196 mod 3359
left =?= right  |  401 =?= 401
Check passed!
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ 
```

```shell
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ ./elgamal -k key -m check -i rsa    
884^280 * 280^420 mod 3359 =?= 1693^11643977109521238678290631291562706216413968527097555853565369948689342216747 mod 3359
left =?= right  |  401 =?= 729
Check failed!
Error: CheckError
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ echo $?
1
➜ chiro@chiro-pc  ~/programs/cryptography-labs/release/dynamic git:(master) ✗ 
```

