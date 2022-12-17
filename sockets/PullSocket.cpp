#include "Sockets.h"
#include <chrono>

pull_socket::pull_socket(const char* end_point)
{
	zmq::context_t context{ 1 };
	socket = zmq::socket_t{ context, zmq::socket_type::push };
	socket.set(zmq::sockopt::linger, SOCKET_LINGER);
	socket.connect(end_point);
}

pull_socket::~pull_socket()
{
	socket.close();
}

FrameData pull_socket::Pull()
{
	zmq::pollitem_t poll[] = { {static_cast<void*>(socket), 0, ZMQ_POLLIN, 0}};
	zmq::message_t message;
	std::chrono::duration<long, std::milli> dur(3000);
	zmq::poll(&poll[0], 1, dur);
	if (poll[0].revents & ZMQ_POLLIN) {
		socket.recv(message);
		auto data = reinterpret_cast<uint8_t*>(message.data());
		std::vector<uint8_t> buf;
		buf.resize(message.size());
		memcpy(buf.data(), data, message.size());
		auto res = deserialize(buf);
		return res;
	}
	return FrameData{};
}