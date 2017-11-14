#![feature(test)]

extern crate test;
extern crate futures;

use futures::{task, Async};
use futures::executor::CurrentThread;
use futures::future::{lazy, poll_fn};

use test::Bencher;

use std::cell::Cell;
use std::rc::Rc;

#[bench]
fn spawn_oneshot(b: &mut Bencher) {
    const ITER: usize = 1000;

    b.iter(move || {
        let cnt = Rc::new(Cell::new(0));

        CurrentThread::block_with_init(|| {
            for _ in 0..ITER {
                let cnt = cnt.clone();
                CurrentThread::spawn(lazy(move || {
                    cnt.set(1 + cnt.get());
                    Ok::<(), ()>(())
                }));
            }
        });

        assert_eq!(cnt.get(), ITER);
    });
}

#[bench]
fn spawn_yield_many(b: &mut Bencher) {
    const YIELDS: usize = 500;
    const TASKS: usize = 20;

    b.iter(move || {
        let cnt = Rc::new(Cell::new(0));

        CurrentThread::block_with_init(|| {
            for _ in 0..TASKS {
                let cnt = cnt.clone();
                let mut rem = YIELDS;

                CurrentThread::spawn(poll_fn(move || {
                    cnt.set(1 + cnt.get());
                    rem -= 1;

                    if rem == 0 {
                        Ok::<_, ()>(().into())
                    } else {
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                }));
            }
        });

        assert_eq!(cnt.get(), YIELDS * TASKS);
    });
}

#[bench]
#[ignore]
fn spawn_daisy(b: &mut Bencher) {
}