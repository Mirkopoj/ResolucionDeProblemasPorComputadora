#include "operaciones.h"
#include <stdio.h>
#include <stdlib.h>

char prety = 0;

int add(matrix mat1, matrix mat2, matrix *res) {
	int ROWS = mat1.rows;
	int COLS = mat1.cols;
	if (ROWS != mat2.rows || COLS != mat2.rows){
		return -1;
	}
	for (int i = 0; i < ROWS; i++) {
		for (int j = 0; j < COLS; j++) {
			res->data[i][j] = mat1.data[i][j] + mat2.data[i][j];
		}
	}
	return 0;
}

void transpose(matrix mat, matrix *res) {
	int ROWS = mat.rows;
	int COLS = mat.cols;
	for (int i = 0; i < ROWS; i++) {
		for (int j = 0; j < COLS; j++) {
			res->data[i][j] = mat.data[j][i];
		}
	}
}

void scalar_mult(matrix mat, int scalar, matrix *res) {
	int ROWS = mat.rows;
	int COLS = mat.cols;
	for (int i = 0; i < ROWS; i++) {
		for (int j = 0; j < COLS; j++) {
			res->data[i][j] = mat.data[i][j] * scalar;
		}
	}
}

int dot(int *vec1, int *vec2, int ROWS) {
	int ret = 0;
	for (int i = 0; i < ROWS; i++) {
		ret += vec1[i] * vec2[i];
	}
	return ret;
}

void matrix_mult(matrix mat1, matrix mat2, matrix *res) {
	int ROWS = mat1.rows;
	int COLS = mat1.cols;
	matrix aux;
	transpose(mat2, &aux);
	for (int i = 0; i < ROWS; i++) {
		for (int j = 0; j < COLS; j++) {
			res->data[i][j] = dot(mat1.data[i], aux.data[j], mat1.rows);
		}
	}
}

void pretty_print(FILE *stream, const char *str) {
	if (prety) {
		fprintf(stream, "%s", str);
	}
}

void matrix_save(FILE *stream, matrix mat) {
	int ROWS = mat.rows;
	int COLS = mat.cols;
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
			fprintf(stream, "%7d", mat.data[i][j]);
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

/*
 * Expects a rows number, a columns number
 * and a set of rows*columns numbers to 
 * populate the matrix with
 *
 * Will return 0 upon success
 * Will return -1 if EOF is reached before matrix gets full
 * Will return 1 if unable to alloc matrix
 * */
int matrix_get(FILE *stream, matrix *mat) {
	if (stream!=stdin) { rewind(stream); }
	//Get matrix size
	if (!stream) { 
		fscanf(stream, "%d", &mat->rows);
		fprintf(stderr, "matrix_get: EOF reached");
		return -1;
	}
	if (!stream) {
		fscanf(stream, "%d", &mat->cols);
		fprintf(stderr, "matrix_get: EOF reached");
		return -1;
	}
	fprintf(stderr, "matrix_get: rows: %d, cols: %d", mat->rows, mat->cols);

	//Allocate memory for specified matrix
	mat->data = (int **)calloc(mat->rows, sizeof(int *));
	if (!mat->data) {
		fprintf(stderr, "matrix_get: Failed to allocate matrix");
		return 1;
	}
	for (int i = 0; i < mat->rows; i++) {
		mat->data[i] = (int *)calloc(mat->cols, sizeof(int));
		if (!mat->data[i]) {
			fprintf(stderr, "matrix_get: Failed to allocate matrix");
			return 1;
		}
	}

	//Populate matrix
	for (int i = 0; i < mat->rows; i++) {
		for (int j = 0; j < mat->cols; j++) {
			if (!stream) {
				fscanf(stream, "%d", &mat->data[i][j]);
				fprintf(stderr, "matrix_get: EOF reached");
				return -1;
			}
		}
	}

	return 0;
}

void matrix_save_setpretty() { prety = 1; }
