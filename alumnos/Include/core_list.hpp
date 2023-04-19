#pragma once
#include "core_node.hpp"

class SinglyLinkedList
{
public:
	SinglyLinkedList();
	SinglyLinkedList(SinglyLinkedList &&) = default;
	SinglyLinkedList(const SinglyLinkedList &) = default;
	SinglyLinkedList &operator=(SinglyLinkedList &&) = default;
	SinglyLinkedList &operator=(const SinglyLinkedList &) = default;
	~SinglyLinkedList();

	void add(CoreNode *new_node);
	int get_count();

protected:
	CoreNode *root;

private:
	int count;
};
