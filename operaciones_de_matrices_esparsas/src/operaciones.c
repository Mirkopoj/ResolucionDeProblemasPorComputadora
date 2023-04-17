#include "../Include/operaciones.h"
#include <stdio.h>
#include <stdlib.h>

char prety = 0;

int add(matrix mat1, matrix mat2, matrix *res) {
	if (rows(mat1) != rows(mat2) || cols(mat1) != cols(mat2)){ return -1; }
	if (rows(mat1) != rows(*res) || cols(mat1) != cols(*res)){
		fprintf(stderr, "Matrix add: Result matrix of invalid size, reallocating\n");
		matrix_free(res);
		set_rows(res, rows(mat1));
		set_cols(res, cols(mat1));
		if (matrix_alloc(res)<0) { return -1; }
	}
	for (int i = 0; i < rows(mat1); i++) {
		for (int j = 0; j < cols(mat1); j++) {
			int a;
			int b;
			coordinate c = {i,j};
			if(get_element(mat1, c, &a)<0) { continue; }
			if(get_element(mat2, c, &b)<0) { continue; }
			int buff = a+b;
			if(set_element(res, c, buff)<0) { return -1; }
		}
	}
	return 0;
}

int scalar_mult(matrix mat, int scalar, matrix *res) {
	if (rows(mat) != rows(*res) || cols(mat) != cols(*res)){
		fprintf(stderr, "Matrix saclar multiply: Result matrix of invalid size, reallocating\n");
		matrix_free(res);
		set_rows(res, rows(mat));
		set_cols(res, cols(mat));
		if (matrix_alloc(res)<0) { return -1; }
	}
	for (int i = 0; i < rows(mat); i++) {
		for (int j = 0; j < cols(mat); j++) {
			int buff;
			coordinate c = {i,j};
			if(get_element(mat, c, &buff)<0) { 
				errase_element(res, c);
				continue; 
			}
			buff *= scalar;
			if(set_element(res, c, buff)<0) { return -1; }
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
	if (cols(mat1)*rows(mat1) != element_count(mat1)) {
		return -1;
	}
	if (cols(mat2)*rows(mat2) != element_count(mat2)) {
		return -1;
	}
	if (cols(mat1) != rows(mat2)) {
		return -1;
	}
	if (rows(mat1) != rows(*res) || cols(mat2) != cols(*res)){
		fprintf(stderr, "Matrix multiply: Result matrix of invalid size, reallocating\n");
		matrix_free(res);
		set_rows(res, rows(mat1));
		set_cols(res, cols(mat2));
		if (matrix_alloc(res)<0) { return -1; }
	}
	matrix aux;
	set_rows(&aux, cols(mat2));
	set_cols(&aux, rows(mat2));
	if (matrix_alloc(&aux)<0) { return -1; }
	transpose(mat2, &aux);
	for (int i = 0; i < rows(*res); i++) {
		for (int j = 0; j < cols(*res); j++) {
			//res->data[i][j] = dot(mat1.data[i], aux.data[j], rows(mat1));
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
	for (int i = 0; i < cols(mat) * num_size; i++) {
		pretty_print(stream, " ");
	}
	pretty_print(stream, "┐");
	fprintf(stream, "\n");

	for (int i = 0; i < rows(mat); i++) {
		pretty_print(stream, "│");
		for (int j = 0; j < cols(mat); j++) {
			int val_xy;
			coordinate c = {i,j};
			if (get_element(mat, c, &val_xy)<0) {
				for (int i = 0; i < num_size; i++) {
					fprintf(stream, " ");
				}	
			} else {
				fprintf(stream, "%7d", val_xy);
			}
		}
		pretty_print(stream, "│");
		fprintf(stream, "\n");
	}

	pretty_print(stream, "└");
	for (int i = 0; i < cols(mat) * num_size; i++) {
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
	int allocation_size = (rows(*mat) * cols(*mat))/10;
	set_data(mat, (struct element*)calloc(allocation_size, sizeof(struct element)));
	if (!get_data(*mat)) {
		fprintf(stderr, "matrix_alloc: Failed to allocate matrix");
		return -1;
	}
	set_num_elements(mat, allocation_size);
	set_assigned_elements(mat, 0);
	return 0;
}

void matrix_free(matrix *mat){
	if (!get_data(*mat)) { return; }
	free(get_data(*mat));
	set_data(mat, NULL);
	set_rows(mat, 0);
	set_cols(mat, 0);
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
	if (feof(stream)) { 
		fprintf(stderr, "matrix_get: EOF reached on rows");
		return -1;
	}
	fscanf(stream, "%d", &mat->rows);
	if (feof(stream)) {
		fprintf(stderr, "matrix_get: EOF reached on cols");
		return -1;
	}
	fscanf(stream, "%d", &mat->cols);

	if (matrix_alloc(mat)<0) { return 1; }

	//Populate matrix
	for (int i = 0; i < mat->rows; i++) {
		for (int j = 0; j < mat->cols; j++) {
			if (feof(stream)) {
				fprintf(stderr, "matrix_get: EOF reached");
				return 0;
			}
			int elem;
			fscanf(stream, "%d %d %d", &i, &j, &elem);
			coordinate c = {i,j};
			set_element(mat, c, elem);
		}
	}

	return 0;
}

void matrix_save_setpretty() { prety = 1; }

void matrix_swap(matrix *mat1, matrix *mat2){
	matrix aux;
	set_cols(&aux, cols(*mat1));
	set_rows(&aux, rows(*mat1));
	set_data(&aux, get_data(*mat1));
	set_cols(mat1, cols(*mat2));
	set_rows(mat1, rows(*mat2));
	set_data(mat1, get_data(*mat2));
	set_cols(mat2, cols(aux));
	set_rows(mat2, rows(aux));
	set_data(mat2, get_data(aux));
}
