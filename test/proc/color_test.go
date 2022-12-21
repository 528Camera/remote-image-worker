package proc

import (
	"os"
	"testing"

	"github.com/AsriFox/remote-image-worker/internal/encod"
	"github.com/AsriFox/remote-image-worker/internal/proc"
)

func TestColorProc(t *testing.T) {
	buf, err := os.ReadFile("image.jpg")
	if err != nil {
		t.Fatal(err)
	}
	img, err := encod.Imdecode(buf)
	if err != nil {
		t.Error(err)
	}
	err = proc.ReduceColors(&img)
	if err != nil {
		t.Error(err)
	}
	buf, err = encod.NewJpegDefaults(50).Imencode(img)
	if err != nil {
		t.Error(err)
	}
	err = os.WriteFile("image_color.jpg", buf, os.ModePerm)
	if err != nil {
		t.Fatal(err)
	}
}
