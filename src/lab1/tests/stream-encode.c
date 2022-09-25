#include "aes.h"

int main(int argc, char *argv[]) {
  if (argc != 2 && argc != 3) {
    Log("Usage: cat source file | lab1-stream-encode key [CBC] > dist file");
    return 0;
  }
  if (argc == 3) mode = MODE_CBC;
  char *key = argv[1];
  if (strlen(key) != 16) {
    Err("Only accept 16 bytes key!");
    return 1;
  }
  char buf[16];
  int done = 0;
  int pArray[4][4] = {0};
  int lastArray[4][4] = {0};
  extendKey(key);
  while (!done) {
    memset(buf, 0, sizeof(buf));
    size_t n = read(0, buf, 16);
    done = n != 16;

    convertToIntArray(buf, pArray);
    if (mode == MODE_CBC)
      xorArray(pArray, lastArray);
    addRoundKey(pArray, 0);  //一开始的轮密钥加
    for (int i = 1; i < 10; i++) {
      subBytes(pArray);  //字节替换
      shiftRows(pArray);  //行移位
      mixColumns(pArray);  //列混合
      addRoundKey(pArray, i);
    }
    subBytes(pArray);  //字节替换
    shiftRows(pArray);  //行移位
    addRoundKey(pArray, 10);
    convertArrayToStr(pArray, buf);
    fwrite(buf, sizeof(buf), 1, stdout);
    if (mode == MODE_CBC)
      memcpy(lastArray, pArray, sizeof(lastArray));
  }
  return 0;
}