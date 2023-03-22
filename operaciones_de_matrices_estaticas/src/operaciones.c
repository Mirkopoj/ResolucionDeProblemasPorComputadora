#include "../Include/operaciones.h"
#include <stdio.h>

char prety = 0;

void add(int mat1[ROWS][COLS], int mat2[ROWS][COLS], int res[ROWS][COLS]) {
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      res[i][j] = mat1[i][j] + mat2[i][j];
    }
  }
}

void transpose(int mat[ROWS][COLS], int res[ROWS][COLS]) {
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      res[i][j] = mat[j][i];
    }
  }
}

void scalar_mult(int mat[ROWS][COLS], int scalar, int res[ROWS][COLS]) {
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      res[i][j] = mat[i][j] * scalar;
    }
  }
}

int dot(int *vec1, int *vec2) {
  int ret = 0;
  for (int i = 0; i < ROWS; i++) {
    ret += vec1[i] * vec2[i];
  }
  return ret;
}

void matrix_mult(int mat1[ROWS][COLS], int mat2[ROWS][COLS], int res[ROWS][COLS]) {
  int aux[ROWS][COLS];
  transpose(mat2, aux);
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      res[i][j] = dot(mat1[i], aux[j]);
    }
  }
}

void pretty_print(FILE *stream, const char *str) {
  if (prety) {
    fprintf(stream, "%s", str);
  }
}

void matrix_save(FILE *stream, int mat[ROWS][COLS]) {
  const int num_size = 7;
  pretty_print(stream, "┌");
  for (int i = 0; i < ROWS * num_size; i++) {
    pretty_print(stream, " ");
  }
  pretty_print(stream, "┐");
  fprintf(stream, "\n");

  for (int i = 0; i < ROWS; i++) {
    pretty_print(stream, "│");
    for (int j = 0; j < COLS; j++) {
      fprintf(stream, "%7d", mat[i][j]);
    }
    pretty_print(stream, "│");
    fprintf(stream, "\n");
  }

  pretty_print(stream, "└");
  for (int i = 0; i < ROWS * num_size; i++) {
    pretty_print(stream, " ");
  }
  pretty_print(stream, "┘");
  fprintf(stream, "\n");
}

void matrix_get(FILE *stream, int mat[ROWS][COLS]) {
  for (int i = 0; i < ROWS; i++) {
    for (int j = 0; j < COLS; j++) {
      fscanf(stream, "%d", &mat[i][j]);
    }
  }
}

void matrix_save_setpretty() { prety = 1; }
