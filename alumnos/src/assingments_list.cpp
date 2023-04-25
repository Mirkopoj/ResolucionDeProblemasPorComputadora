#include "../Include/assingments_list.hpp"

AssingmentNode *AssingmentsList::get_root(){
	return (AssingmentNode *)root;
}

void AssingmentsList::set_root(AssingmentNode * new_root){
	root = new_root;
}

bool AssingmentsList::remove(std::string assingment_name){
	AssingmentNode *node = get_root();
	if (node == nullptr) { return false; }
	if (node->get_assingment().get_name() == assingment_name) {
		set_root(node->remove());
		return true;
	}
	if (node->get_next() == nullptr) { return false; }
	while (node->get_next()->get_assingment().get_name() != assingment_name) {
		node = node->get_next();
		if (node->get_next() == nullptr) { return false; }
	}
	node->append(node->get_next()->remove());
	return true;
}

std::optional<Assingment> AssingmentsList::get_assingment(std::string assingment_name){
	AssingmentNode *node = get_root();
	if (node == nullptr) { return std::nullopt; }
	while (node->get_assingment().get_name() != assingment_name) {
		node = node->get_next();
		if (node->get_next() == nullptr) { return std::nullopt; }
	}
	return node->get_assingment();
}
