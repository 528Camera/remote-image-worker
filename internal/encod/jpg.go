package encod

import (
	"gocv.io/x/gocv"
)

type Jpeg struct {
	quality          int
	is_progressive   bool
	is_optimized     bool
	restart_interval int
	luma_quality     int
	chroma_quality   int
}

func NewJpegDefaults(quality int) *Jpeg {
	return &Jpeg{
		quality:          quality,
		is_progressive:   false,
		is_optimized:     true,
		restart_interval: 0,
		luma_quality:     -1,
		chroma_quality:   -1,
	}
}

func (self Jpeg) params() []int {
	var params []int
	params = append(params, gocv.IMWriteJpegQuality, self.quality)
	if self.is_progressive {
		params = append(params, gocv.IMWriteJpegProgressive, 1)
	}
	if self.is_optimized {
		params = append(params, gocv.IMWriteJpegOptimize, 1)
	}
	if self.restart_interval != 0 {
		params = append(params, gocv.IMWriteJpegRstInterval, self.restart_interval)
	}
	if self.luma_quality != -1 {
		params = append(params, gocv.IMWriteJpegLumaQuality, self.luma_quality)
	}
	if self.chroma_quality != -1 {
		params = append(params, gocv.IMWriteJpegChromaQuality, self.chroma_quality)
	}
	return params
}

func (self Jpeg) Imencode(img gocv.Mat) ([]byte, error) {
	return Imencode(img, gocv.JPEGFileExt, self.params())
}
