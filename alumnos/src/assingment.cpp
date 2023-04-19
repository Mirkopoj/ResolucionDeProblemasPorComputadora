#include "../Include/assingment.hpp"
#include <string>
#include <iostream>

Assingment::Assingment(std::string n, int g):name(n), grade(g){}

Assingment::~Assingment(){}

std::string Assingment::get_name(){
	return name;
}

int Assingment::get_mark(){
	return grade;
}

AssingmentNode::AssingmentNode(Assingment a):assingment(a), CoreNode(){}

AssingmentNode::~AssingmentNode(){}

AssingmentNode *AssingmentNode::get_next(){
	return (AssingmentNode *)CoreNode::get_next();
}

AssingmentNode *AssingmentNode::remove(){
	return (AssingmentNode *)CoreNode::remove();
}

Assingment AssingmentNode::get_assingment(){
	return assingment;
}

void AssingmentNode::print() {
	std::cout<<"->{"<<assingment.get_name()<<", "<<assingment.get_mark()<<"}";
}
