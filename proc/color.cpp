#include "proc.h"

typedef cv::Point3_<uchar> Pixel;

inline uchar reduce_color(uchar color) {
	return (color & 0xC0) + 32;
}

cv::Mat reduce_colors(cv::Mat image) {
	int size = image.size().width * image.size().height;
	cv::Mat result;
	image.copyTo(result);
	result.forEach<Pixel>([&result](Pixel &pixel, const int* pos) {
		result.at<Pixel>(pos) = Pixel(
			reduce_color(pixel.x),
			reduce_color(pixel.y),
			reduce_color(pixel.z)
		);
	});
	return result;

	auto data = result.ptr();
#pragma omp parallel
	for (int i = 0; i < size; i++) {
		data[i] = reduce_color(data[i]);
	}
	return result;
}