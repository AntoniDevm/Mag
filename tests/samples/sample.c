#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("Usage: %s <name> <age>",argv[0]);
    return 1;
  };
  int age = atoi(argv[2]);
  if (age < 18) {
    printf("You must be over 18 to enter. Sorry :(");
    return 1;
  }
  printf("Welocme: %s", argv[1]);
}

