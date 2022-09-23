#include "aes.h"

int main(int argc, char const *argv[]) {
  test = &test_default;
  char key[17];
  int klen;

  int cos = 0;
  printf("************************$声明信息$****************************\n");
  printf("版权声明：未经授权，禁止传播、使用和用于商业用途\n");
  printf("使用说明：本程序是AES密码演示程序。\n");
  printf("**********************$声明信息$******************************\n");
  printf("================AES密码算法程序演示================\n\n");

  while (1) {
    if (test->key == NULL) {
      printf("请输入16个字符的密钥：\n");
      getString(key, 17);
    } else {
      strcpy(key, test->key);
    }
    klen = strlen(key);
    if (klen != 16) {
      printf("请输入16个字符的密钥,当前密钥的长度为%d\n", klen);
    } else {
      printf("你输入的密钥为：%s\n", key);
      break;
    }
  }

  aesStrToFile(key);
  if (!test->auto_decode) {
    printf("是否开始解密,1解密，2退出\n");
    scanf("%d", &cos);
  } else {
    cos = 1;
  }
  if (cos == 1) {
    deAesFile(key);
  } else {
    return 0;
  }
  if (!test->auto_exit)
    MUXDEF(WIN32, system("pause"), system("read"));
  return 0;
}