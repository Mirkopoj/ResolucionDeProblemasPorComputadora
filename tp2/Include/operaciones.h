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
int transpose(matrix mat, matrix *res);
int scalar_mult(matrix mat, int scalar, matrix *res);
int matrix_mult(matrix mat1, matrix mat2, matrix *res);
void matrix_save_setpretty(void);
int matrix_alloc(matrix *mat);
void matrix_free(matrix *mat);
void matrix_swap(matrix *mat1, matrix *mat2);

#endif
