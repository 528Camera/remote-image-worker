package proc

import (
	"gocv.io/x/gocv"
)

func reduceColor(v uint8) uint8 {
	return (v & 0xC0) + 32
}

func ReduceColors(image *gocv.Mat) error {
	data, err := image.DataPtrUint8()
	if err != nil {
		return err
	}
	for i := 0; i < len(data); i++ {
		data[i] = reduceColor(data[i])
	}
	return nil
}
