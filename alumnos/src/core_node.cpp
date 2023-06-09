#include "../Include/core_node.hpp"
#include <iostream>

CoreNode::CoreNode(...){
	next = nullptr;
}

CoreNode::~CoreNode(){
	delete next;
}

CoreNode* CoreNode::get_next(){
	return next;
}

void CoreNode::append(CoreNode *new_next){
	next = new_next;
}

CoreNode* CoreNode::remove(){
	CoreNode *aux = get_next();
	append(nullptr);
	delete this;
	return aux;
}
