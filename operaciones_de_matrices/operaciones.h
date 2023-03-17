#ifndef OPS_MATRIX_H
#define OPS_MATRIX_H

#define DIM 5

void fill(int mat[DIM][DIM]);
void show(int mat[DIM][DIM]);
void add(int mat1[DIM][DIM], int mat2[DIM][DIM], int res[DIM][DIM]);
void transpose(int mat[DIM][DIM], int res[DIM][DIM]);
void scalar_mult(int mat[DIM][DIM], int scalar, int res[DIM][DIM]);
void matrix_mult(int mat1[DIM][DIM], int mat2[DIM][DIM], int res[DIM][DIM]);

#endif // !DEBUG
