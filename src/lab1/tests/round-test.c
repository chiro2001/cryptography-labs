#include "aes.h"

int main(int argc, char *argv[]) {
  // for (int k = MODE_ECB; k <= MODE_CBC; k++) {
  //   mode = k;
  //   for (int i = 0; i < sizeof(test_set) / sizeof(test_t); i++) {
  //     test = &test_set[i];
  //     Log("Test round %d: mode [ %s ] key [ %s ] data [ %s ]", i, mode == MODE_CBC ? "CBC" : "ECB", test->key, test->plain_text);
  //     aesStrToFile(test->key);
  //     deAesFile(test->key);
  //   }
  // }
  for (int k = MODE_CBC; k <= MODE_CBC; k++) {
    mode = k;
    test = &test_set[0];
    Log("Test round %d: mode [ %s ] key [ %s ] data [ %s ]", 0, mode == MODE_CBC ? "CBC" : "ECB", test->key, test->plain_text);
    aesStrToFile(test->key);
    deAesFile(test->key);
  }
  return 0;
}