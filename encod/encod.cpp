#include "encod.h"
#include "format_jpg.h"

cv::Mat imdecode(const std::vector<uchar> buf) {
	int flags = cv::IMREAD_COLOR;
	auto buffer = cv::InputArray(buf);
	auto img = cv::imdecode(buffer, flags);
	return img;
}

std::vector<uchar> imencode(cv::Mat img, cv::String ext, const std::vector<int> params) {
	auto buf = std::vector<uchar>();
	if (cv::imencode(ext, img, buf, params)) {
		return buf;
	}
	return std::vector<uchar>();
}

cv::String ext(Format format) {
	switch (format) {
	case jpg:
		return cv::String(".jpg");
	}
	return cv::String();
}

std::vector<int> params(Format format) {
	switch (format) {
	case jpg:
		return jpg_params(50);
	}
	return std::vector<int>();
}

std::vector<uchar> imencode(cv::Mat img, Format format) {
	auto _ext = ext(format);
	auto _params = params(format);
	return imencode(img, _ext, _params);
}