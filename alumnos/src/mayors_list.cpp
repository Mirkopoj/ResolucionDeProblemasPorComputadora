#include "../Include/mayors_list.hpp"

MayorNode *MayorsList::get_root(){
	return (MayorNode *)root;
}

void MayorsList::set_root(MayorNode * new_root){
	root = new_root;
}

bool MayorsList::remove(Mayor mayor){
	MayorNode *node = get_root();
	if (node == nullptr) { return false; }
	if (node->get_mayor() == mayor) {
		set_root(node->remove());
		return true;
	}
	if (node->get_next() == nullptr) { return false; }
	while (node->get_next()->get_mayor() != mayor) {
		node = node->get_next();
		if (node->get_next() == nullptr) { return false; }
	}
	node->append(node->get_next()->remove());
	return true;
}

std::optional<Mayor> MayorsList::get_mayor(Mayor mayor){
	MayorNode *node = get_root();
	if (node == nullptr) { return std::nullopt; }
	while (node->get_mayor() != mayor) {
		node = node->get_next();
		if (node->get_next() == nullptr) { return std::nullopt; }
	}
	return node->get_mayor();
}
