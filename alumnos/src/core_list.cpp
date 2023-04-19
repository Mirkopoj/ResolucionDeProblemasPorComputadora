#include "../Include/core_list.hpp"

SinglyLinkedList::SinglyLinkedList(){
	count = 0;
	root = nullptr;
}

SinglyLinkedList::~SinglyLinkedList(){
	delete root;
}

void SinglyLinkedList::add(CoreNode *new_node){
	CoreNode *last = root;
	while (last->get_next() != nullptr) {
		last = last->get_next();
	}
	last->append(new_node);
	count++;
}

int SinglyLinkedList::get_count(){
	return count;
}
