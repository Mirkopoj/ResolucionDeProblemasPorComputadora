#pragma once
#include "core_list.hpp"
#include "mayors.hpp"
#include <string>
#include <optional>

class MayorsList:public SinglyLinkedList
{
public:
	using SinglyLinkedList::SinglyLinkedList;

	bool remove(Mayor mayor);
	std::optional<Mayor> get_mayor(Mayor mayor);

private:
	MayorNode *list;

	MayorNode *get_root();
	void set_root(MayorNode *new_root);
};
