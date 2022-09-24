#include "aes.h"

#define Test(name) Log("===== Test " name " =====")
#define Pass(name) Log("===== Pass " name " =====")
#define TestF(func) Log("===== Test " #func " =====")
#define PassF(func) Log("===== Pass " #func " =====")
#define LogI(i) Log(#i " = 0x%x", i)

int data_matrix[4][4];
int data_array[4];
int res_matrix[4][4];

void init_data() {
  int *p = (int *) data_matrix;
  while (p < (int *) data_matrix + 16) {
    *p = (int) (p - (int *) data_matrix);
    p++;
  }
  p = data_array;
  while (p < (int *) data_array + 4) {
    *p = (int) (p - (int *) data_array);
    p++;
  }
}

int main() {
  Test("Simple");
  Log("Simple output...");
  Pass("Simple");

  init_data();
  TestF(shiftArrayOneStep);
  show_array(data_array);
  shiftArrayOneStep(data_array, 1);
  show_array(data_array);
  PassF(shiftArrayOneStep);

  init_data();
  TestF(leftLoop4int);
  show_array(data_array);
  leftLoop4int(data_array, 2);
  show_array(data_array);
  PassF(leftLoop4int);

  init_data();
  TestF(subBytes);
  show_matrix(data_matrix);
  subBytes(data_matrix);
  show_matrix(data_matrix);
  PassF(subBytes);
  TestF(deSubBytes);
  show_matrix(data_matrix);
  deSubBytes(data_matrix);
  show_matrix(data_matrix);
  PassF(deSubBytes);

  TestF(T);
  int num = 0, round = 0;
  printf("T(%d, %d) = %x\n", num, round, T(num, round));
  PassF(T);

  int a[4][4] = {
          0x02, 0x03, 0x01, 0x01,
          0x01, 0x02, 0x03, 0x01,
          0x01, 0x01, 0x02, 0x03,
          0x03, 0x01, 0x01, 0x02
  };
  int b[4][4] = {
          0xd1, 0x85, 0x1a, 0xf9,
          0x93, 0x1b, 0xf7, 0x10,
          0xca, 0xa8, 0xb6, 0x45,
          0x40, 0x8f, 0xf5, 0x20
  };
  int c[4][4] = {0};
  TestF(matGFMul);
  matGFMul(a, b, c);
  show_matrix(c);
  PassF(matGFMul);

  init_data();
  TestF(mixColumns);
  mixColumns(data_matrix);
  show_matrix(data_matrix);
  PassF(mixColumns);
  TestF(deMixColumns);
  deMixColumns(data_matrix);
  show_matrix(data_matrix);
  PassF(deMixColumns);

  TestF(shiftByteOneStep);
  Log("0x11223344 -> 0x%08x", shiftByteOneStep(0x11223344));
  PassF(shiftByteOneStep);

  init_data();
  int d[4][4] = {
          0x0a, 0x04, 0x07, 0x09,
          0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00
  };
  int pkey[44] = {0x02050d05, 0};
  TestF(addRoundKey);
  memcpy(w, pkey, sizeof(pkey));
  show_array(w);
  addRoundKey(d, 0);
  show_matrix(d);
  PassF(addRoundKey);
  Test("bit calc");
  LogI((0x73656375 >> 24) & 0xff);
  Pass("bit calc");

  TestF(GFMul);
  printf("GFMul(1, 1) = %x\n", GFMul(1, 1));
  PassF(GFMul);

  TestF(GFMul2);
  printf("GFMul2(1) = %x\n", GFMul2(1));
  PassF(GFMul2);
  return 0;
}