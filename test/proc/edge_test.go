package proc

import (
	"os"
	"testing"

	"github.com/AsriFox/remote-image-worker/internal/encod"
	"github.com/AsriFox/remote-image-worker/internal/proc"
)

func TestDetectEdges(t *testing.T) {
	buf, err := os.ReadFile("image.jpg")
	if err != nil {
		t.Fatal(err)
	}
	img, err := encod.Imdecode(buf)
	if err != nil {
		t.Error(err)
	}
	edg := proc.DetectEdges(&img)
	buf, err = encod.NewJpegDefaults(100).Imencode(edg)
	if err != nil {
		t.Error(err)
	}
	err = os.WriteFile("image_edge.jpg", buf, os.ModePerm)
	if err != nil {
		t.Fatal(err)
	}
}
