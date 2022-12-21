package encod

import (
	"gocv.io/x/gocv"
)

func Imdecode(buf []byte) (gocv.Mat, error) {
	return gocv.IMDecode(buf, gocv.IMReadAnyColor)
}

func Imencode(img gocv.Mat, ext gocv.FileExt, params []int) ([]byte, error) {
	buf, err := gocv.IMEncode(ext, img)
	if err != nil {
		return nil, err
	}
	return buf.GetBytes(), nil
}
