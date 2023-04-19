#include "../Include/core_list.hpp"
#include <iostream>

SinglyLinkedList::SinglyLinkedList(){
	count = 0;
	root = nullptr;
}

SinglyLinkedList::~SinglyLinkedList(){
	delete root;
}

void SinglyLinkedList::add(CoreNode *new_node){
	if (!root) {
		root = new_node;
		count++;
		return;
	}
	CoreNode *last = root;
	while (last->get_next()) {
		last = last->get_next();
	}
	last->append(new_node);
	count++;
}

int SinglyLinkedList::get_count(){
	return count;
}

void SinglyLinkedList::print(){
	if (!root) { return; }
	CoreNode *iter = root;
	if(iter) iter->print();
	while (iter->get_next()) {
		iter = iter->get_next();
		iter->print();
	}
	std::cout << "\n";
}
