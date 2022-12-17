#include "pb.h"

std::vector<uint8_t> serialize(FrameData fd)
{
	std::vector<uint8_t> res;
	res.resize(fd.ByteSizeLong());
	fd.SerializeToArray(res.data(), res.size());
	return res;
}

FrameData deserialize(std::vector<uint8_t> buff)
{
	FrameData res;
	res.ParseFromArray(buff.data(), buff.size());
	return res;
}