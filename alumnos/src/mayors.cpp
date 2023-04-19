#include "../Include/mayors.hpp"

MayorNode::MayorNode(Mayor m):mayor(m), CoreNode(){}

MayorNode::~MayorNode(){}

MayorNode* MayorNode::get_next(){
	return (MayorNode *)CoreNode::get_next();
}

MayorNode* MayorNode::remove(){
	return (MayorNode *)CoreNode::remove();
}

Mayor MayorNode::get_mayor(){
	return mayor;
}
