#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/*
 * See ./src/bin/13.rs for reference.
 *
 * Chatjipitty Solution. I wanted to test if custom bytestream parser makes any
 * sense at all. Takes about 80µs on average for an iteration so about 5 times
 * slower than the custom streaming one (15µs).
 *
 * Compiled with `gcc -Ofast -march=native main.c`
 * */

int main(int argc, char *argv[]) {
  long long int sum = 0, a, b, c, d, e, f, x, y, denominator;
  const int iterations = 100000; // Number of iterations
  double total_time = 0.0;

  // Open the file
  FILE *fp = fopen("./data/inputs/13.txt", "r");
  if (!fp) {
    perror("Failed to open file");
    return 1;
  }

  // Get file size
  fseek(fp, 0, SEEK_END);
  long file_size = ftell(fp);
  rewind(fp);

  // Allocate memory for the file content
  char *file_content = malloc(file_size + 1);
  if (!file_content) {
    perror("Failed to allocate memory");
    fclose(fp);
    return 1;
  }

  // Read the file content into memory
  fread(file_content, 1, file_size, fp);
  fclose(fp);
  file_content[file_size] = '\0'; // Null-terminate the string

  // Repeat the algorithm for the specified number of iterations
  for (int i = 0; i < iterations; ++i) {
    sum = 0; // Reset sum for each iteration

    // Create a copy of the file content for this iteration
    char *file_copy = strdup(file_content);
    if (!file_copy) {
      perror("Failed to duplicate file content");
      free(file_content);
      return 1;
    }

    char *line = strtok(file_copy, "\n");

    clock_t start = clock(); // Start timing

    while (line) {
      // Parse the three lines for a single group
      if (sscanf(line, "Button A: X+%lld, Y+%lld", &a, &d) == 2) {
        line = strtok(NULL, "\n");
        sscanf(line, "Button B: X+%lld, Y+%lld", &b, &e);
        line = strtok(NULL, "\n");
        sscanf(line, "Prize: X=%lld, Y=%lld", &c, &f);

        c += 10000000000000LL;
        f += 10000000000000LL;

        denominator = (a * e - b * d);
        x = (c * e - b * f) / denominator;
        y = (a * f - c * d) / denominator;

        if (x >= 0 && y >= 0 && a * x + b * y == c && d * x + e * y == f)
          sum += 3 * x + y;
      }
      line = strtok(NULL, "\n"); // Move to the next line
    }

    clock_t end = clock(); // End timing

    total_time += (double)(end - start) / CLOCKS_PER_SEC; // Accumulate time

    free(file_copy); // Free the copy of file content
  }

  // Free allocated memory
  free(file_content);

  // Calculate average time in microseconds
  double average_time_us = (total_time / iterations) * 1e6;

  printf("Sum: %lld\n", sum);
  printf("Average Time: %.2f µs\n", average_time_us);

  return 0;
}
