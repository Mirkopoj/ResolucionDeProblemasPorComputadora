#include "operaciones.h"
#include <stdio.h>

void add(int mat1[DIM][DIM], int mat2[DIM][DIM], int res[DIM][DIM]){
	for(int i=0;i<DIM;i++){
		for(int j=0;j<DIM;j++){
			res[i][j] = mat1[i][j] + mat2[i][j];
		}
	}
}

void transpose(int mat[DIM][DIM], int res[DIM][DIM]){
	for(int i=0;i<DIM;i++){
		for(int j=0;j<DIM;j++){
			res[i][j] = mat[j][i];
		}
	}
}

void scalar_mult(int mat[DIM][DIM], int scalar, int res[DIM][DIM]){
	for(int i=0;i<DIM;i++){
		for(int j=0;j<DIM;j++){
			res[i][j] = mat[i][j] * scalar;
		}
	}
}

int dot(int *vec1, int *vec2){
	int ret = 0;
	for (int i=0; i<DIM; i++) {
		ret += vec1[i]*vec2[i];
	}
	return ret;
}

void matrix_mult(int mat1[DIM][DIM], int mat2[DIM][DIM], int res[DIM][DIM]){
	int aux[DIM][DIM];
	transpose(mat2, aux);
	for(int i=0;i<DIM;i++){
		for(int j=0;j<DIM;j++){
			res[i][j] = dot(mat1[i], aux[j]);
		}
	}
}

void matrix_save(FILE *stream, int mat[DIM][DIM]){
	const int num_size = 7;
	fprintf(stream, "┌");
	for (int i=0; i<DIM*num_size; i++) {
		fprintf(stream, " ");
	}
	fprintf(stream, "┐\n");

	for (int i=0; i<DIM; i++) {
		fprintf(stream, "│");
		for (int j=0; j<DIM; j++) {
			fprintf(stream, "%7d", mat[i][j]);
		}
		fprintf(stream, "│\n");
	}

	fprintf(stream, "└");
	for (int i=0; i<DIM*num_size; i++) {
		fprintf(stream, " ");
	}
	fprintf(stream, "┘\n");
}

void matrix_get(FILE *stream, int mat[DIM][DIM]){
	for (int i=0; i<DIM; i++) {
		for (int j=0; j<DIM; j++) {
			fscanf(stream, "%d", &mat[i][j]);
		}
	}
}
