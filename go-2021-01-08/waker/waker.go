package waker

import (
	"fmt"
	"sync"
	"time"
)

// Object is a proxy for our notification objects.  It contains a time and any sort of payload.
type Object struct {
	T    time.Time
	Name string
}

// Repo is where we store objects that come in.  Here, we'll be using a simple wrapper around a []Object
// This implementation protects Repo inside a Mutex, so it is not expected to be threadsafe.
type Repo interface {
	// Add inserts an object to to the repo.
	Add(Object)
	// FetchReady returns all objects that are ready to be processed
	FetchReady() []Object
	FetchNext() *Object
}

// Waker sleeps until something is ready to be delivered, then "wakes up" and calls the provided callback
// on all objects that are ready.
type Waker struct {
	callback func(Object)
	mutex    *sync.Mutex
	repo     Repo
	nextTime time.Time
	timer    *time.Timer
}

// defaultInterval is how long we sleep before waking if no objects are ready.
// Theoretically, this could be infinite, but it can't hurt to wake up and verify
// that nothing is ready.
const defaultInterval = time.Minute

// NewWaker instantiates a Waker with the provided Repo and callback.
func NewWaker(repo Repo, callback func(Object)) *Waker {
	return &Waker{
		callback: callback,

		mutex:    &sync.Mutex{},
		repo:     repo,
		nextTime: time.Now().Add(defaultInterval),
		timer:    time.NewTimer(defaultInterval),
	}
}

// Run processes the Waker forever.
//
// We include a small lag to keep from flogging the processor
// when there are no objects ready to process.  There should be
// a way to do this without locks, with a blocking channel read,
// but I'm not sure how to avoid the race condition between
// timer.Stop() and timer.Reset() when shortening the timeout
// if we don't lock.
//
// There are some potentially good suggestions at:
// https://www.reddit.com/r/golang/comments/f16kqy/the_right_way_to_restart_timetimer/
func (w *Waker) Run() {
	for {
		w.mutex.Lock()
		select {
		case <-w.timer.C:
			w.handleCallbacks()
		default:
		}
		w.mutex.Unlock()
		time.Sleep(500 * time.Microsecond)
	}
}

// AddObject inserts an object into the Repo, and resets the
// timer if the new Object should be processed before the next
// wake time.
//
// An AddMany method would prevent a lot of mutex/timer flogging.
func (w *Waker) AddObject(o Object) {
	w.mutex.Lock()
	w.repo.Add(o)
	if o.T.Before(w.nextTime) {
		w.nextTime = o.T
		// Due to using mutexes, we know the timer isn't being blocked.
		w.timer.Stop()
		w.timer.Reset(time.Until(w.nextTime))
	}
	w.mutex.Unlock()

}

// handleCallbacks fetches all the objects that are ready for processing,
// and calls the callback on each one.
//
// *Thread Safety*
//
// w.mutex must be locked before calling this method.
func (w *Waker) handleCallbacks() {
	fmt.Println("handling callbacks")
	ready := w.repo.FetchReady()

	for _, obj := range ready {
		w.callback(obj)
	}
	t := w.getNextTime()
	w.nextTime = t
	// Since the mutex is locked, we know the timer's channel isn't being read from elsewhere
	w.timer.Stop()
	w.timer.Reset(time.Until(w.nextTime))
}

// getNextTime returns the next time we should wake.
func (w *Waker) getNextTime() time.Time {
	// Adding a short duration to the now variable could prevent too-rapid iteration.
	now := time.Now()
	o := w.repo.FetchNext()
	if o == nil {
		// Nothing to process. Sleep for a while.
		return now.Add(defaultInterval)
	} else {
		if o.T.Before(now) {
			return now
		} else {
			return o.T
		}
	}
}
