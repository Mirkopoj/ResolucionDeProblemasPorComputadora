#include "../Include/operaciones.h"
#include <stdio.h>
#include <stdlib.h>

char prety = 0;

int add(matrix mat1, matrix mat2, matrix *res) {
	if (mat1.rows != mat2.rows || mat1.cols != mat2.cols){ return -1; }
	if (mat1.rows != res->rows || mat1.cols != res->cols){
		fprintf(stderr, "Matrix add: Result matrix of invalid size, reallocating\n");
		if(res) { matrix_free(res); }
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
		if(res) { matrix_free(res); }
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

int scalar_mult(matrix mat, float scalar, matrix *res) {
	if (mat.rows != res->rows || mat.cols != res->cols){
		fprintf(stderr, "Matrix saclar multiply: Result matrix of invalid size, reallocating\n");
		if(res) { matrix_free(res); }
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

int dot(float *vec1, float *vec2, int rows) {
	float ret = 0;
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
		if(res) { matrix_free(res); }
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
			fprintf(stream, "%7.2f", mat.data[i][j]);
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
	mat->data = (float **)calloc(mat->rows, sizeof(float *));
	if (!mat->data) {
		fprintf(stderr, "matrix_alloc: Failed to allocate matrix");
		return -1;
	}
	for (int i = 0; i < mat->rows; i++) {
		mat->data[i] = (float *)calloc(mat->cols, sizeof(float));
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
			fscanf(stream, "%f", &mat->data[i][j]);
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

void restar_filas(float *f1, float *f2, int cols){
	for (int i=0; i<cols; i++) {
		f1[i] -= f2[i];
	}
}

void fila_por_escalar(float *f, float n, int cols){
	for (int i=0; i<cols; i++) {
		f[i] *= n;
	}
}

int matrix_clone(matrix src, matrix *dst){
	if (src.rows != dst->rows || src.cols != dst->cols){
		fprintf(stderr, "Matrix clone: Result matrix of invalid size, reallocating\n");
		if(dst) { matrix_free(dst); }
		dst->rows=src.rows;
		dst->cols=src.cols;
		if (matrix_alloc(dst)<0) { return -1; }
	}
	for (int i=0; i<src.rows; i++) {
		for (int j=0; j<src.cols; j++) {
			dst->data[i][j] = src.data[i][j];
		}
	}
	return 0;
}

int matrix_diag_core(matrix mat, matrix *ret, matrix *solutions, char preserve){
	int max_iter = mat.cols>mat.rows? mat.rows:mat.cols;
	if (matrix_clone(mat, ret)<0) { return -1; }
	for (int j=0; j<max_iter; j++) {
		for (int i=0; i<mat.rows; i++) {
			if (i!=j) {
				float obj = ret->data[j][j];
				float target = ret->data[i][j];
				if (obj != 0.0 && target != 0.0) {
					if (solutions) {
						fila_por_escalar(solutions->data[j], target/obj, solutions->cols);
						restar_filas(solutions->data[i], solutions->data[j], solutions->cols);
						if (preserve) { fila_por_escalar(solutions->data[j], obj/target, solutions->cols); }
					}
					fila_por_escalar(ret->data[j], target/obj, mat.cols);
					restar_filas(ret->data[i], ret->data[j], mat.cols);
					if (preserve) { fila_por_escalar(ret->data[j], obj/target, mat.cols); }
				}
			}
		}
	}
	return 0;
}

int matrix_solve(matrix mat, matrix *ret, matrix *solutions){
	return matrix_diag_core(mat, ret, solutions, 0);
}

int matrix_diag(matrix mat, matrix *ret){
	return matrix_diag_core(mat, ret, NULL, 0);
}

int matrix_make_identity(matrix *inv, int size){
	if (inv->rows != size || inv->cols != size){
		if (inv) { matrix_free(inv); }
		inv->rows = size;
		inv->cols = size;
		if (matrix_alloc(inv)<0) { return -1; }
	}
	for (int i=0; i<size; i++) {
		for (int j=0; j<size; j++) {
			inv->data[i][j] = j==i? 1.0:0.0;
		}
	}
	return 0;
}

void matrix_det_2_3(matrix mat, float *ret){
	*ret = 0;
	for (int i=0; i<mat.rows; i++) {
		float mults = 1;
		float multr = 1;
		for (int j=0; j<mat.cols; j++) {
			mults *= mat.data[(i+j)%mat.rows][j];
			multr *= mat.data[(i+j)%mat.rows][mat.cols-j];
		}
		*ret += mults;
		*ret -= multr;
	}
}

int matrix_det_core(matrix mat, float *ret, matrix *aux){
	if (mat.rows != mat.cols) { return -1; }
	if (mat.rows < 4) {
		matrix_det_2_3(mat, ret);
		return 0;
	}
	if (aux->rows != mat.cols || aux->cols != mat.rows){
		if (aux) { matrix_free(aux); }
		aux->rows = mat.cols;
		aux->cols = mat.rows;
		if (matrix_alloc(aux)<0) { return -1; }
	}
	transpose(mat, aux);
	matrix aux2;
	aux2.rows = aux->rows;
	aux2.cols = aux->cols;
	if (matrix_alloc(&aux2)<0) { return -1; }
	if (matrix_diag_core(*aux, &aux2, NULL, 1)) { return -1; }
	*ret = 1.0;
	for (int i=0; i<aux2.rows; i++) {
		*ret *= aux2.data[i][i];
	}
	matrix_free(&aux2);
	return 0;
}

int matrix_det(matrix mat, float *ret){
	matrix aux;
	aux.rows = mat.rows;
	aux.cols = mat.cols;
	if (matrix_alloc(&aux)<0) { return -1; }
	return matrix_det_core(mat, ret, &aux);
}

/*
 *Returns 1 if matrix not inversible
 *Returns -1 in case of failure
 * */
int matrix_inv(matrix mat, matrix *inv){
	float det;
	if (matrix_det_core(mat, &det, inv)<0) { return 1; }
	if (det == 0.0) { return 1; }
	if (matrix_make_identity(inv, mat.rows)<0) { return -1; }
	matrix ret_aux;
	ret_aux.rows = mat.rows;
	ret_aux.cols = mat.cols;
	if (matrix_alloc(&ret_aux)<0) { return -1; }
	int failure = matrix_diag_core(mat, &ret_aux, inv, 0);
	if (!failure) {
		for (int i; i<inv->rows; i++) {
			fila_por_escalar(inv->data[i], 1.0/ret_aux.data[i][i], inv->cols);
		}
	}	
	if (!ret_aux.data) {
		matrix_free(&ret_aux);
	}
	return failure;
}
