#include "format_jpg.h"
#include <vector>
#include <opencv2/imgcodecs.hpp>

std::vector<int> jpg_params(
	int quality,
	bool progressive,
	bool optimize,
	int restart_interval,
	int luma_quality,
	int chroma_quality
) {
	auto params = std::vector<int>();
	params.push_back(cv::IMWRITE_JPEG_QUALITY);
	params.push_back(quality);
	if (progressive) {
		params.push_back(cv::IMWRITE_JPEG_PROGRESSIVE);
		params.push_back(1);
	}
	if (optimize) {
		params.push_back(cv::IMWRITE_JPEG_OPTIMIZE);
		params.push_back(1);
	}
	if (restart_interval != DISABLE_RESTART_INTERVAL) {
		params.push_back(cv::IMWRITE_JPEG_RST_INTERVAL);
		params.push_back(restart_interval);
	}
	if (luma_quality != DISABLE_LUMA_QUALITY) {
		params.push_back(cv::IMWRITE_JPEG_LUMA_QUALITY);
		params.push_back(luma_quality);
	}
	if (chroma_quality != DISABLE_CHROMA_QUALITY) {
		params.push_back(cv::IMWRITE_JPEG_CHROMA_QUALITY);
		params.push_back(chroma_quality);
	}
	return params;
}