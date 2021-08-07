use std::future::Future;
use std::time::Duration;
use std::pin::Pin;
use std::task;


static WAKER_VTABLE: task::RawWakerVTable = task::RawWakerVTable::new(
    raw_waker_clone,
    raw_waker_noop,
    raw_waker_noop,
    raw_waker_noop,
);

unsafe fn raw_waker_clone(data: *const ()) -> task::RawWaker {
    task::RawWaker::new(data, &WAKER_VTABLE)
}

unsafe fn raw_waker_noop(data: *const ()) {}

fn sleep(dur: Duration) -> SleepUntil {
    SleepUntil {
        t: std::time::Instant::now() + dur
    }
}
struct SleepUntil {
    t: std::time::Instant,
}

impl Future for SleepUntil {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<()> {
        if std::time::Instant::now() > self.t {
            task::Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            task::Poll::Pending
        }
    }
}

fn main() {
    let raw_waker = unsafe { raw_waker_clone(&() as *const ()) };
    let waker = unsafe { task::Waker::from_raw(raw_waker) };
    let mut context = task::Context::from_waker(&waker);
    let mut fut = sleep_then_fetch();

    let s = loop {
        let fut = unsafe { Pin::new_unchecked(&mut fut) };
        match fut.poll(&mut context) {
            task::Poll::Pending => std::thread::sleep(Duration::from_millis(50)),
            task::Poll::Ready(s) => break s,
        }
    };
    println!("{}!", s);
}

async fn sleep_then_fetch() -> String {
    sleep(Duration::from_secs(5)).await;
    String::from("good morning, waker")
}
