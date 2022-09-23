#include <math.h>
#include "aes.h"

#define Test(name) Log("===== Test " name " =====")
#define Pass(name) Log("===== Pass " name " =====")
#define TestF(func) Log("===== Test " #func " =====")
#define PassF(func) Log("===== Pass " #func " =====")

void show_array_(char *name, int *array, int size) {
  printf("array %s[0..%d]:\t", name, size);
  for (int i = 0; i < size; i++) printf("%d\t", array[i]);
  puts("");
}

void show_matrix_(char *name, int *matrix, int size) {
  printf("matrix %s[0..%d]:\n\t", name, size);
  int len = (int) sqrt(size);
  for (int i = 0; i < len; i++) {
    for (int j = 0; j < len; j++)
      printf("%2x\t", matrix[i * len + j]);
    printf("\n\t");
  }
  puts("");
}

#define show_array(array) show_array_(#array, (int*)(array), sizeof(array) / sizeof(int))
#define show_matrix(matrix) show_matrix_(#matrix, (int*)(matrix), sizeof(matrix) / sizeof(int))

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
  return 0;
}