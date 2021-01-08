package main

import (
	"fmt"
	"time"

	"github.com/jcdyer/spikes/go-2021-01-08/slicerepo"
	"github.com/jcdyer/spikes/go-2021-01-08/waker"
)

func objAt(n int64) waker.Object {
	return waker.Object{
		T:    time.Now().Add(time.Duration(n) * time.Second),
		Name: fmt.Sprintf("In %d", n),
	}
}

func main() {
	now := time.Now()
	fmt.Println("starting at ", now)
	immediate := waker.Object{
		T:    now,
		Name: "immediate",
	}
	in2 := objAt(2)
	in5 := objAt(5)
	in10 := objAt(10)

	in5b := waker.Object{
		T:    in5.T,
		Name: "in 5 b",
	}
	in5c := waker.Object{
		T:    in5.T.Add(100 * time.Microsecond),
		Name: "in 5 c",
	}
	callback := func(o waker.Object) {
		fmt.Printf(" object \"%s\": %s\n", o.Name, time.Since(o.T))
	}
	waker := waker.NewWaker(&slicerepo.SliceRepo{}, callback)

	waker.AddObject(immediate)
	waker.AddObject(in5)
	waker.AddObject(in5b)
	waker.AddObject(in5c)
	waker.AddObject(in10)

	go func() {
		waker.Run()
	}()

	time.Sleep(1 * time.Second)

	waker.AddObject(in2)

	time.Sleep(12 * time.Second)
}
