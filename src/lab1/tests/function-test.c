#include "aes.h"

#define Test(name) Log("===== Test " name " =====")
#define Pass(name) Log("===== Pass " name " =====")
#define TestF(func) Log("===== Test " #func " =====")
#define PassF(func) Log("===== Pass " #func " =====")

void show_array_(char *name, int *array, int size) {
  printf("array %s[0..%d]:\t", name);
  for (int i = 0; i < size; i++) printf("%d\t", array[i]);
  puts("");
}

#define show_array(array) show_array_(#array, (int*)(array), sizeof(array) / sizeof(int))

int data_array[4][4];
int data_line[4];

void init_data() {
  int *p = (int *) data_array;
  while (p < (int *) data_array + 16) {
    *p = p - (int *) data_array;
    p++;
  }
  p = data_line;
  while (p < (int *) data_line + 4) {
    *p = p - (int *) data_line;
    p++;
  }
}

int main() {
  Test("Simple");
  Log("Simple output...");
  Pass("Simple");

  init_data();
  TestF(shiftArrayOneStep);
  show_array(data_line);
  shiftArrayOneStep(data_line, 1);
  show_array(data_line);
  PassF(shiftArrayOneStep);

  init_data();
  TestF(leftLoop4int);
  show_array(data_line);
  leftLoop4int(data_line, 2);
  show_array(data_line);
  PassF(leftLoop4int);
  return 0;
}