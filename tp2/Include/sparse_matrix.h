#ifndef SPARSE_MATRIX
#define SPARSE_MATRIX

typedef struct coordinate {
	int row;
	int col;
} coordinate;

typedef struct matrix {
	int rows;
	int cols;
	struct element{
		coordinate c;
		int datum;
	}* data;
	int num_elements;
	int assigned_elements;
} matrix ;

int get_element(matrix mat, coordinate cord, int *elem);
int set_element(matrix mat, coordinate cord, int elem);
void errase_element(matrix mat, coordinate cord);
int rows(matrix mat);
int cols(matrix mat);
int element_count(matrix mat);
int element_capacity(matrix mat);

#endif // !SPARSE_MATRIX
