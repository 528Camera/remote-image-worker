#pragma once
#include <zmq.hpp>
#include "..\pb\pb.h"

#define SOCKET_LINGER 100

class push_socket
{
private:
	zmq::socket_t socket;
public:
	push_socket();
	void Connect(const char* end_point);
	void Bind(const char* end_point);
	bool Push(FrameData Fd);
	~push_socket();
};

class pull_socket
{
private:
	zmq::socket_t socket;
public:
	pull_socket(const char * end_point);
	FrameData Pull();
	~pull_socket();
};