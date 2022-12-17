#pragma once
#include <vector>

const int DISABLE_RESTART_INTERVAL = 0;
const int DISABLE_LUMA_QUALITY = -1;
const int DISABLE_CHROMA_QUALITY = -1;

std::vector<int> jpg_params(
	int quality,
	bool progressive = false,
	bool optimize = false,
	int restart_interval = DISABLE_RESTART_INTERVAL,
	int luma_quality = DISABLE_LUMA_QUALITY,
	int chroma_quality = DISABLE_CHROMA_QUALITY
);