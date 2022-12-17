#pragma once
#include "frame_data.pb.h"

std::vector<uint8_t> serialize(FrameData fd);

FrameData deserialize(std::vector<uint8_t> buff);