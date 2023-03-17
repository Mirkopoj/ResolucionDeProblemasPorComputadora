#include "operaciones.h"
#include "stdio.h"

int main(void) {

	int mat1[DIM][DIM];
	int mat2[DIM][DIM];

	fill(mat1);
	fill(mat2);

	printf("Mat1:\n");
	show(mat1);
	printf("Mat2:\n");
	show(mat2);

	int ret[DIM][DIM];

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
