#pragma once
#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>
#include <vector>

cv::Mat imdecode(std::vector<uchar> buf);

std::vector<uchar> imencode(cv::Mat img, cv::String ext, const std::vector<int> params);

enum Format { jpg, png, webp };

std::vector<uchar> imencode(cv::Mat img, Format format);
