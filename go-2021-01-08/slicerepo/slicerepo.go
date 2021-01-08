package slicerepo

import (
	"time"

	"github.com/jcdyer/spikes/go-2021-01-08/waker"
)

// SliceRepo implements the waker.Repo interface
type SliceRepo struct {
	objects []waker.Object
}

func (s *SliceRepo) Add(o waker.Object) {
	s.objects = append(s.objects, o)
}

func (s *SliceRepo) FetchReady() []waker.Object {
	now := time.Now()
	ready := []waker.Object{}
	unexpired := []waker.Object{}
	for _, obj := range s.objects {
		if !obj.T.After(now) {
			ready = append(ready, obj)
		} else {
			unexpired = append(unexpired, obj)
		}
	}
	s.objects = unexpired
	return ready
}

func (s *SliceRepo) FetchNext() *waker.Object {
	var next *waker.Object
	for i := range s.objects {
		obj := s.objects[i]
		if next == nil || obj.T.Before(next.T) {
			next = &obj
		}
	}
	return next
}

var _ waker.Repo = &SliceRepo{}
