#include "../Include/sparse_matrix.h"
#include <stdlib.h>

int ord(coordinate cord){
	return cord.row + cord.col;
}

int same_cord(coordinate a, coordinate b){
	if (a.row != b.row) {
		return 0;
	}
	if (a.col != b.col) {
		return 0;
	}
	return 1;
}

int bin_search(matrix mat, coordinate cord, int *rptr){
	int elems = element_count(mat);
	if (!elems) { return -1; }
	int sptr = elems/2;
	int ref = ord(cord);
	while (1) {	
		if (same_cord(mat.data[sptr].c, cord)) {
			*rptr = sptr;
			return 0;
		}
		int sptr_prev = sptr;
		if (ref < ord(mat.data[sptr].c)) {
			sptr /= 2;
		} else {
			sptr *= 1.5;
		}
		if (sptr_prev == sptr) {
			*rptr = sptr;
			return -1;
		}
	}
}

int enlarge(matrix mat){
	struct element *new_data = realloc(mat.data, element_capacity(mat)*2);
	if (!new_data) { return -1; }
	mat.data = new_data;
	mat.num_elements *= 2;
	return 0;
}

void shrunk(matrix mat){
	struct element *new_data = realloc(mat.data, element_capacity(mat)/2);
	if (!new_data) { return; }
	mat.data = new_data;
	mat.num_elements /= 2;
}

void increase_count(matrix mat){
	mat.assigned_elements++;
}

void decrement_count(matrix mat){
	mat.assigned_elements--;
}

void element_swap(struct element a, struct element b){
	struct element aux = a;
	a = b;
	b = aux;
}

//	Element getters
int get_element_bin(matrix mat, coordinate cord, int *elem){
	int sptr;
	if (bin_search(mat, cord, &sptr) < 0) { return -1; }
	*elem = mat.data[sptr].datum;
	return 0;
}

int (*element_getter)(matrix, coordinate, int*) = get_element_bin;

int get_element(matrix mat, coordinate cord, int *elem){
	return element_getter(mat, cord, elem);
}

//	Element setters
int set_element_bin(matrix mat, coordinate cord, int elem){
	if (element_capacity(mat) == element_count(mat)){
		if (enlarge(mat) < 0) { return -1; }
	}
	int sptr;
	if (bin_search(mat, cord, &sptr) < 0) {
		for (int i=sptr+1; i<=element_count(mat); i++) {
			element_swap(mat.data[sptr], mat.data[i]);
		}
	}
	mat.data[sptr].c = cord;
	mat.data[sptr].datum = elem;
	increase_count(mat);
	return 0;
}

int (*element_setter)(matrix, coordinate, int) = set_element_bin;

int set_element(matrix mat, coordinate cord, int elem){
	return element_setter(mat, cord, elem);
}

// Element deleters
void delete_element_bin(matrix mat, coordinate cord){
	int sptr;
	if (bin_search(mat, cord, &sptr) < 0) { return; }
	for (int i=sptr; i<element_count(mat)-1; i++) {
		element_swap(mat.data[i], mat.data[i+1]);
	}
	decrement_count(mat);
	if (element_capacity(mat) > 2*element_count(mat)) {
		shrunk(mat);	
	}
}

void (*element_deleter)(matrix, coordinate) = delete_element_bin;

void errase_element(matrix mat, coordinate cord){
	element_deleter(mat, cord);
}

int rows(matrix mat){
	return mat.rows;
}

int cols(matrix mat){
	return mat.cols;
}

int element_count(matrix mat){
	return mat.assigned_elements;
}

int element_capacity(matrix mat){
	return mat.num_elements;
}
