#pragma once
#include "assingment.hpp"
#include "core_list.hpp"
#include <string>
#include <optional>

class AssingmentsList:public SinglyLinkedList
{
public:
	using SinglyLinkedList::SinglyLinkedList;

	bool remove(std::string assingment_name);
	std::optional<Assingment> get_assingment(std::string assingment_name);

private:
	AssingmentNode *list;

	AssingmentNode *get_root();
	void set_root(AssingmentNode *new_root);
};
