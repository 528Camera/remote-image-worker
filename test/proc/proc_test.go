package proc

import (
	"os"
	"testing"

	"github.com/AsriFox/remote-image-worker/internal/encod"
	"github.com/AsriFox/remote-image-worker/internal/proc"
	"gocv.io/x/gocv"
)

func TestProc(t *testing.T) {
	buf, err := os.ReadFile("image.jpg")
	if err != nil {
		t.Fatal(err)
	}
	img, err := encod.Imdecode(buf)
	if err != nil {
		t.Error(err)
	}
	edg := proc.DetectEdges(&img)
	gocv.CvtColor(edg, &edg, gocv.ColorGrayToBGR)
	err = proc.ReduceColors(&img)
	if err != nil {
		t.Error(err)
	}
	imgr := img.Clone()
	gocv.AddWeighted(img, 1.0, edg, -1.0, 0.0, &imgr)
	buf, err = encod.NewJpegDefaults(50).Imencode(imgr)
	if err != nil {
		t.Error(err)
	}
	err = os.WriteFile("image_proc.jpg", buf, os.ModePerm)
	if err != nil {
		t.Fatal(err)
	}
}
