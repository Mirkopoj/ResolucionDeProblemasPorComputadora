#include "../Include/operaciones.h"
#include <stdio.h>
#include <stdlib.h>

char prety = 0;

int add(matrix mat1, matrix mat2, matrix *res) {
	if (mat1.rows != mat2.rows || mat1.cols != mat2.cols){ return -1; }
	if (mat1.rows != res->rows || mat1.cols != res->cols){
		fprintf(stderr, "Matrix add: Result matrix of invalid size, reallocating\n");
		matrix_free(res);
		res->rows=mat1.rows;
		res->cols=mat1.cols;
		if (matrix_alloc(res)<0) { return -1; }
	}
	for (int i = 0; i < mat1.rows; i++) {
		for (int j = 0; j < mat1.cols; j++) {
			res->data[i][j] = mat1.data[i][j] + mat2.data[i][j];
		}
	}
	return 0;
}

int transpose(matrix mat, matrix *res) {
	if (mat.rows != res->cols || mat.cols != res->rows) {
		fprintf(stderr, "Matrix transpose: Result matrix of invalid size, reallocating\n");
		matrix_free(res);
		res->rows=mat.cols;
		res->cols=mat.rows;
		if (matrix_alloc(res)<0) { return -1; }
	}
	for (int i = 0; i < mat.rows; i++) {
		for (int j = 0; j < mat.cols; j++) {
			res->data[j][i] = mat.data[i][j];
		}
	}
	return 0;
}

int scalar_mult(matrix mat, int scalar, matrix *res) {
	if (mat.rows != res->rows || mat.cols != res->cols){
		fprintf(stderr, "Matrix saclar multiply: Result matrix of invalid size, reallocating\n");
		matrix_free(res);
		res->rows=mat.rows;
		res->cols=mat.cols;
		if (matrix_alloc(res)<0) { return -1; }
	}
	for (int i = 0; i < mat.rows; i++) {
		for (int j = 0; j < mat.cols; j++) {
			res->data[i][j] = mat.data[i][j] * scalar;
		}
	}
	return 0;
}

int dot(int *vec1, int *vec2, int rows) {
	int ret = 0;
	for (int i = 0; i < rows; i++) {
		ret += vec1[i] * vec2[i];
	}
	return ret;
}

int matrix_mult(matrix mat1, matrix mat2, matrix *res) {
	if (mat1.cols != mat2.rows) {
		return -1;
	}
	if (mat1.rows != res->rows || mat2.cols != res->cols){
		fprintf(stderr, "Matrix multiply: Result matrix of invalid size, reallocating\n");
		matrix_free(res);
		res->rows=mat1.rows;
		res->cols=mat2.cols;
		if (matrix_alloc(res)<0) { return -1; }
	}
	matrix aux;
	aux.rows = mat2.cols;
	aux.cols = mat2.rows;
	if (matrix_alloc(&aux)<0) { return -1; }
	transpose(mat2, &aux);
	for (int i = 0; i < res->rows; i++) {
		for (int j = 0; j < res->cols; j++) {
			res->data[i][j] = dot(mat1.data[i], aux.data[j], mat1.rows);
		}
	}
	matrix_free(&aux);
	return 0;
}

void pretty_print(FILE *stream, const char *str) {
	if (prety) {
		fprintf(stream, "%s", str);
	}
}

void matrix_save(FILE *stream, matrix mat) {
	const int num_size = 7;
	pretty_print(stream, "┌");
	for (int i = 0; i < mat.cols * num_size; i++) {
		pretty_print(stream, " ");
	}
	pretty_print(stream, "┐");
	fprintf(stream, "\n");

	for (int i = 0; i < mat.rows; i++) {
		pretty_print(stream, "│");
		for (int j = 0; j < mat.cols; j++) {
			fprintf(stream, "%7d", mat.data[i][j]);
		}
		pretty_print(stream, "│");
		fprintf(stream, "\n");
	}

	pretty_print(stream, "└");
	for (int i = 0; i < mat.cols * num_size; i++) {
		pretty_print(stream, " ");
	}
	pretty_print(stream, "┘");
	fprintf(stream, "\n");
}

/*
 * Allocates memotry a given matrix
 * Expects rows and colums to be set previously
 *
 * will return -1 on allocation fail*/
int matrix_alloc(matrix *mat){
	mat->data = (int **)calloc(mat->rows, sizeof(int *));
	if (!mat->data) {
		fprintf(stderr, "matrix_alloc: Failed to allocate matrix");
		return -1;
	}
	for (int i = 0; i < mat->rows; i++) {
		mat->data[i] = (int *)calloc(mat->cols, sizeof(int));
		if (!mat->data[i]) {
			fprintf(stderr, "matrix_alloc: Failed to allocate matrix");
			matrix_free(mat);
			return -1;
		}
	}
	return 0;
}

void matrix_free(matrix *mat){
	if (!mat->data) { return; }
	for (int i=0; i<mat->rows; i++) {
		if (!mat->data[i]) { break; }
		free(mat->data[i]);
		mat->data[i] = NULL;
	}
	free(mat->data);
	mat->data = NULL;
	mat->rows=0;
	mat->cols=0;
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
		fprintf(stderr, "matrix_get: EOF reached");
		return -1;
	}
	fscanf(stream, "%d", &mat->rows);
	if (!stream) {
		fprintf(stderr, "matrix_get: EOF reached");
		return -1;
	}
	fscanf(stream, "%d", &mat->cols);

	if (matrix_alloc(mat)<0) { return 1; }

	//Populate matrix
	for (int i = 0; i < mat->rows; i++) {
		for (int j = 0; j < mat->cols; j++) {
			if (!stream) {
				fprintf(stderr, "matrix_get: EOF reached");
				return -1;
			}
			fscanf(stream, "%d", &mat->data[i][j]);
		}
	}

	return 0;
}

void matrix_save_setpretty() { prety = 1; }

void matrix_swap(matrix *mat1, matrix *mat2){
	matrix aux;
	aux.cols = mat1->cols;
	aux.rows = mat1->rows;
	aux.data = mat1->data;
	mat1->cols = mat2->cols;
	mat1->rows = mat2->rows;
	mat1->data = mat2->data;
	mat2->cols = aux.cols;
	mat2->rows = aux.rows;
	mat2->data = aux.data;
}
