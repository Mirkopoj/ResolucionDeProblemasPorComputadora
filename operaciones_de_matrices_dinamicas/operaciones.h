#ifndef OPS_MATRIX_H
#define OPS_MATRIX_H
#include <stdio.h>

typedef struct matrix {
	int rows;
	int cols;
	int **data;
} matrix ;

void matrix_save(FILE *stream, matrix mat);
int matrix_get(FILE *stream, matrix *mat);
int add(matrix mat1, matrix mat2, matrix *res);
void transpose(matrix mat, matrix *res);
void scalar_mult(matrix mat, int scalar, matrix *res);
void matrix_mult(matrix mat1, matrix mat2, matrix *res);
void matrix_save_setpretty(void);

#endif // !DEBUG
