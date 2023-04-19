#pragma once
#include "core_node.hpp"

enum class Mayor {
	ElectronisEngineering,
	TelecomunicationsEngineering,
	ComputingEngineering,
	EnviromentalEngineering,
};

class MayorNode:public CoreNode
{
public:
	MayorNode(Mayor m);
	MayorNode(MayorNode &&) = default;
	MayorNode(const MayorNode &) = default;
	MayorNode &operator=(MayorNode &&) = default;
	MayorNode &operator=(const MayorNode &) = default;
	~MayorNode();

	MayorNode *get_next();
	MayorNode *remove();
	Mayor get_mayor();

private:
	Mayor mayor;
};
