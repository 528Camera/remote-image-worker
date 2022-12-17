#include "proc.h"
#include <opencv2/core.hpp>
#include <opencv2/imgproc.hpp>

cv::Mat detecte_edges(cv::Mat image)
{
	cv::Mat res;
	cv::Canny(image, res, 50, 200);
	cv::cvtColor(res, res, cv::COLOR_GRAY2BGR);
	return res;
}