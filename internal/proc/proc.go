package proc

import "gocv.io/x/gocv"

func ProcessImage(img *gocv.Mat) (*gocv.Mat, error) {
	edg := DetectEdges(img)
	gocv.CvtColor(edg, &edg, gocv.ColorGrayToBGR)
	if err := ReduceColors(img); err != nil {
		return nil, err
	}
	imgr := img.Clone()
	gocv.AddWeighted(*img, 1.0, edg, -1.0, 0.0, &imgr)
	return &imgr, nil
}
