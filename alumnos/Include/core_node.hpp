#pragma once

class CoreNode
{
public:
	CoreNode(...);
	CoreNode(CoreNode &&) = default;
	CoreNode(const CoreNode &) = default;
	CoreNode &operator=(CoreNode &&) = default;
	CoreNode &operator=(const CoreNode &) = default;
	~CoreNode();

	CoreNode *get_next();
	void append(CoreNode *next);;
	CoreNode *remove();

private:
	CoreNode *next;
};
