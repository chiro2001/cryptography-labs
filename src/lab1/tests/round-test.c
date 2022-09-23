#include "aes.h"

int main(int argc, char *argv[]) {
  for (int i = 0; i < sizeof(test_set) / sizeof(test_t); i++) {
    test = &test_set[i];
    Log("Test round %d: key [ %s ] data [ %s ]", i, test->key, test->plain_text);
    aesStrToFile(test->key);
    deAesFile(test->key);
  }
  return 0;
}