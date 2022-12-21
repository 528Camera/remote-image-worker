package proc

import (
	"gocv.io/x/gocv"
)

func DetectEdges(image *gocv.Mat) gocv.Mat {
	matCanny := gocv.NewMat()
	gocv.Canny(*image, &matCanny, 50, 200)
	return matCanny
}
