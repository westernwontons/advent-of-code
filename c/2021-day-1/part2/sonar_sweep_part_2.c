#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define CHUNK 1024
#define get_array_size(x) sizeof(x) / sizeof(x[0])

/**
 * Get the size of a file
 */
long get_file_size(FILE *file) {
  long file_size;

  fseek(file, 0, SEEK_END);
  file_size = ftell(file);
  rewind(file);

  return file_size;
}

/**
 * Three element window into an array
 */
int window(int array[], size_t len, int i) {
  int chunk[] = {array[i], array[i + 1], array[i + 2]};
  return *chunk;
}

int main() {
  char *filename = "dataset.txt";
  FILE *file = NULL;
  long file_size;

  char *buffer;
  file = fopen(filename, "r");

  if (file) {

    long file_size = get_file_size(file);

    buffer = malloc(file_size + 1);

    if (!buffer) {
      fclose(file);
      free(buffer);
      fputs("memory allocation failed", stderr);
      exit(1);
    }

    /**
     * If the amount of read bytes is not at least one, handle it as an error
     */
    if (fread(buffer, file_size, 1, file) != 1) {
      fclose(file);
      free(buffer);
      fputs("read failed", stderr);
      exit(1);
    }

    int array[sizeof(buffer)];
    int array_len = sizeof(array) / sizeof(array[0]);
    printf("%lu\n", sizeof(buffer));

    for (int i = 0; i < array_len - 1; i++) {
      array[i] = buffer[i];
    }

    for (int i = 0; i < array_len - 1; i++) {
      int chunk = window(array, array_len, i);
      printf("%d\n", chunk);
    }

    fclose(file);
    free(buffer);
  }
}

// footnote: I give up on C, Rust is the way
