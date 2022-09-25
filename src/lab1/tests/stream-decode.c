#include "aes.h"

int main(int argc, char *argv[]) {
  if (argc != 2 && argc != 3) {
    Log("Usage: cat file | lab1-stream-decode key [CBC] > decoded file");
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
  int cArray[4][4] = {0};
  int lastArray[4][4] = {0};
  int lastArray2[4][4] = {0};
  extendKey(key);
  while (1) {
    memset(buf, 0, sizeof(buf));
    size_t n = read(0, buf, 16);
    done = n != 16;
    if (done) break;

    convertToIntArray(buf, cArray);
    if (mode == MODE_CBC) {
      memcpy(lastArray2, lastArray, sizeof(lastArray));
      memcpy(lastArray, cArray, sizeof(lastArray));
    }
    addRoundKey(cArray, 10);
    deShiftRows(cArray);  //行移位
    deSubBytes(cArray);  //字节替换
    for (int i = 1; i < 10; i++) {
      addRoundKey(cArray, 10 - i);
      deMixColumns(cArray);  //列混合
      deShiftRows(cArray);  //行移位
      deSubBytes(cArray);  //字节替换
    }
    addRoundKey(cArray, 0);  //一开始的轮密钥加
    if (mode == MODE_CBC)
      xorArray(cArray, lastArray2);
    convertArrayToStr(cArray, buf);
    fwrite(buf, sizeof(buf), 1, stdout);
  }
  return 0;
}