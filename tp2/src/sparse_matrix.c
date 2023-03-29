#include "../Include/sparse_matrix.h"

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

int get_element_bin(matrix mat, coordinate cord, int *elem){
	int elems = element_count(mat);
	if (!elems) { return -1; }
	int ref = ord(cord);
	char buscando = 1;	
	int sptr = elems/2;
	while (1) {	
		if (same_cord(mat.data[sptr].c, cord)) {
			*elem = mat.data[sptr].datum;
			return 0;
		}
		int sptr_prev = sptr;
		if (ord(cord) < ord(mat.data[sptr].c)) {
			sptr /= 2;
		} else {
			sptr *= 1.5;
		}
		if (sptr_prev == sptr) {
			return 1;
		}
	}
}

int (*element_getter)(matrix, coordinate, int*) = get_element_bin;

int get_element(matrix mat, coordinate cord, int *elem){
	return element_getter(mat, cord, elem);
}
