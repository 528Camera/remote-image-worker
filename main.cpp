#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include <iostream>
#include "sockets/Sockets.h"
#include "proc/proc.h"
#include "pb/pb.h"
#include "encod/encod.h"
#include <fstream>
#include <string>
#include <memory>
#include <stdexcept>

using namespace std;

int main(int argc, char** argv) {
	if (argc <  3) {
		cout << "Usage: " << argv[0] << " Pull_end_point Push_end_point [dump_path]" << endl;
		return -1;
	}
	string pull_end_point = argv[1], push_end_point = argv[2], dump_path;
	if (argc == 4) dump_path = argv[3];
	pull_socket puller(pull_end_point.c_str());
	push_socket pusher;
	pusher.Connect(push_end_point.c_str());
	int retry_count = 0;
	while (retry_count < 10)
	{
		auto Fd = puller.Pull();
		if (Fd.version() != 1) {
			retry_count++;
			cout << "Pull Failed!\n";
			continue;
		}
		vector<uint8_t> buf;
		buf.resize(Fd.image().size());
		auto image_data = Fd.image().data();
		memcpy(buf.data(), reinterpret_cast<const uchar*>(image_data), Fd.image().size());
		auto image = imdecode(buf);
		if (image.empty()) {
			cout << "Failed to decode image!\n";
			continue;
		}
		auto r1 = reduce_colors(image);
		auto r2 = detecte_edges(image);
		cv::addWeighted(r1, 1.0, r2, -1.0, 0, image);
		buf = imencode(image, Format::jpg);
		if (!dump_path.empty()) {
			int size_s = std::snprintf(nullptr, 0, "\\frame_%d", Fd.frame_index()) + 1;
			if (size_s <= 0) { throw std::runtime_error("Error during formatting."); }
			auto size = static_cast<size_t>(size_s);
			std::unique_ptr<char[]> buff(new char[size]);
			std::snprintf(buff.get(), size, "\\frame_%d", Fd.frame_index());
			auto path = std::string(buff.get(), buff.get() + size - 1);

			std::ofstream outfile(dump_path + path, std::ios::binary);
			int length = buf.size();
			char* buffer = new char[length];
			memcpy(reinterpret_cast<char*>(buffer), buf.data(), length * sizeof(uchar));
			outfile.write(buffer, length);
			outfile.close();
			continue;
		}
		Fd.set_image(buf.data(), buf.size());
		pusher.Push(Fd);
	}
	cv::Mat image;
	image = cv::imread(argv[1], cv::IMREAD_COLOR);
	if (image.empty()) {
		cout << "Could not open the image file" << endl;
		return -1;
	}
	cv::namedWindow("Display", cv::WINDOW_AUTOSIZE);
	cv::imshow("Display", image);
	cv::waitKey(0);
	return 0;
}