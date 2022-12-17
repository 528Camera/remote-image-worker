#include "Sockets.h"

push_socket::push_socket()
{
	zmq::context_t context{1};
	socket = zmq::socket_t{ context, zmq::socket_type::push };
	socket.set(zmq::sockopt::linger, SOCKET_LINGER);
}

push_socket::~push_socket()
{
	socket.close();
}

void push_socket::Connect(const char* end_point)
{
	socket.connect(end_point);
}

void push_socket::Bind(const char* end_point)
{
	socket.bind(end_point);
}

bool push_socket::Push(FrameData Fd)
{
	auto data = serialize(Fd);	
	auto buf = zmq::buffer(data);
	auto res = socket.send(buf);
	return res.value() == buf.size();
}