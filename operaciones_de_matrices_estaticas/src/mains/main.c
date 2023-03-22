#include "../../Include/operaciones.h"
#include <stdio.h>
#include <math.h>

void show(int mat[ROWS][COLS]);
void fill(int mat[ROWS][COLS]);

int main(void) {

	int mat1[ROWS][COLS];
	int mat2[ROWS][COLS];

	fill(mat1);
	fill(mat2);

	printf("Mat1:\n");
	show(mat1);
	printf("Mat2:\n");
	show(mat2);

	int ret[ROWS][COLS];

	printf("Mat1 + Mat2:\n");
	add(mat1, mat2, ret);
	show(ret);

	printf("[Mat1]^T:\n");
	transpose(mat1, ret);
	show(ret);

	printf("Mat1 * 5:\n");
	scalar_mult(mat1, 5, ret);
	show(ret);

	printf("Mat1 * Mat2:\n");
	matrix_mult(mat1, mat2, ret);
	show(ret);

	return 0;
}

void fill(int mat[ROWS][COLS]){
	for(int i=0;i<ROWS;i++){
		for(int j=0;j<COLS;j++){
			mat[i][j] = pow(i,j);
		}
	}
}

void show(int mat[ROWS][COLS]){
	matrix_save(stdout, mat);
}

