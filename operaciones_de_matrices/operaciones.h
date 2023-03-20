#ifndef OPS_MATRIX_H
#define OPS_MATRIX_H
#include <stdio.h>

#define ROWS 5
#define COLS 5

void matrix_save(FILE *stream, int mat[ROWS][COLS]);
void matrix_get(FILE *stream, int mat[ROWS][COLS]);
void add(int mat1[ROWS][COLS], int mat2[ROWS][COLS], int res[ROWS][COLS]);
void transpose(int mat[ROWS][COLS], int res[ROWS][COLS]);
void scalar_mult(int mat[ROWS][COLS], int scalar, int res[ROWS][COLS]);
void matrix_mult(int mat1[ROWS][COLS], int mat2[ROWS][COLS], int res[ROWS][COLS]);
void matrix_save_setpretty(void);

#endif // !DEBUG
