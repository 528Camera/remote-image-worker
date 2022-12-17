#include "pch.h"
#include "CppUnitTest.h"
#include <string>
#include <vector>
#include <fstream>
#include "../encod/encod.h"
#include "../proc/proc.h"
#include <iostream>
#include "..\pb\pb.h"
#include <algorithm>
#include "..\sockets\Sockets.h"

#define TEST_CASE_DIRECTORY GetDirectoryName(__FILE__)

std::string GetDirectoryName(std::string path) {
	const size_t last_slash_idx = path.rfind('\\');
	if (std::string::npos != last_slash_idx) {
		return path.substr(0, last_slash_idx + 1);
	}
	return "";
}

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace tests
{
	inline std::vector<uchar> read_sample(std::string file_name) {
		std::ifstream infile(TEST_CASE_DIRECTORY + file_name, std::ios::binary);
		infile.seekg(0, infile.end);
		size_t length = infile.tellg();
		infile.seekg(0, infile.beg);
		Assert::IsTrue(length > 0);
		char* buf = new char[length];
		infile.read(buf, length);
		infile.close();
		auto buffer = std::vector<uchar>(length);
		memcpy(buffer.data(), reinterpret_cast<uchar*>(buf), length * sizeof(uchar));
		delete[] buf;
		return buffer;
	}

	inline void write_result(std::string file_name, std::vector<uchar>& buffer) {
		std::ofstream outfile(TEST_CASE_DIRECTORY + file_name, std::ios::binary);
		int length = buffer.size();
		char* buf = new char[length];
		memcpy(reinterpret_cast<char*>(buf), buffer.data(), length * sizeof(uchar));
		outfile.write(buf, length);
		outfile.close();
	}

	TEST_CLASS(tests)
	{
	public:
		TEST_METHOD(RunReduceColors)
		{
			auto buf = read_sample("image.jpg");
			auto img = imdecode(buf);
			Assert::IsFalse(img.empty(), L"Decode failed");
			auto r1 = reduce_colors(img);
			auto r2 = detecte_edges(img);
			Assert::IsFalse(r1.empty(), L"Color reduce failed");
			Assert::IsFalse(r2.empty(), L"Failed detecte edges");
			cv::addWeighted(r1, 1.0, r2, -1.0, 0, img);
			buf = imencode(img, Format::jpg);
			Assert::IsFalse(buf.empty(), L"Encode failed");
			write_result("image_proc.jpg", buf);
		}

		TEST_METHOD(RunPbPack) 
		{
			auto buf = read_sample("image.jpg");
			
			FrameData Fd;
			Fd.set_version(1);
			Fd.set_frame_index(5);
			Fd.set_image(buf.data(), buf.size());
			auto content = serialize(Fd);
			Assert::IsFalse(content.empty(), L"Serialize failed");
			write_result("image.dat", content);
		}

		TEST_METHOD(TestPbUnpack)
		{
			auto buf = read_sample("image.dat");
			auto Fd = deserialize(buf);
			Assert::AreEqual(Fd.version(), 1);
			Assert::AreEqual(Fd.frame_index(), 5);
			auto img = read_sample("image.jpg");
			buf.resize(Fd.image().size());
			auto image = Fd.image().data();
			memcpy(buf.data(), reinterpret_cast<const uchar*>(image), Fd.image().size());
			//auto pair = std::mismatch(img.begin(), img.end(), buf.begin());
			Assert::IsTrue(buf == img);
		}

		/*TEST_METHOD(TestPushPull)
		{
			push_socket pusher;
			pusher.Bind("tcp://*:5557");

			pull_socket puller("tcp://127.0.0.1:5557");
			for (int i = 0; i < 10; i++) {
				FrameData Fd;
				Fd.set_version(1);
				Fd.set_frame_index(i);
				if (!pusher.Push(Fd)) {
					Assert::Fail(L"Failed to Push");
				}
				auto Fdd = puller.Pull();
				Assert::AreEqual(Fdd.version(), 1);
			}
		}*/
	};

}
