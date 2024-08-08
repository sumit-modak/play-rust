use std::{cell::{RefCell, OnceCell}, rc::Rc};
use std::future::Future;
use std::{pin::Pin, collections::VecDeque};

// Task alias
type Task = Pin<Box<dyn Future<Output = ()>>>;

// our executor struct owns our task queue
#[derive(Default)]
pub struct Executor {
    // Using a RefCell because we'll need interior mutability :O
    tasks: RefCell<VecDeque<Task>>
}

thread_local! {
    // Our thread local executor. Will be initialised later.
    // Uses an Rc because it will need multiple owners
    static EXECUTOR: OnceCell<Rc<Executor>> = OnceCell::new();
}

/// Spawns a future in our thread-local executor
pub fn spawn(fut: impl Future<Output = ()>) {
    EXECUTOR.with(|e| e.get().unwrap().spawn(fut));
}

impl Executor {
    /// registers this executor onto the current thread
    fn register(self: &Rc<Self>) {
        EXECUTOR.with(|e| e.set(self.clone()));
    }

    pub fn spawn(&self, fut: impl Future<Output = ()>) {
        self.tasks.borrow_mut().push_back(Box::pin(fut));
    }

    /// Waits for the future to complete
    pub fn block_on<F: Future>(&self, fut: F) -> F::Output {
        Self::register(&Rc::new(*self)); // we're now in executor land

        // a way to store the output of the future, which will also signal
        // that we are done
        let output: Rc<RefCell<Option<F::Output>>> = Rc::new(RefCell::new(None));

        self.spawn(async {
            let output2 = Rc::clone(&output);
            *output2.borrow_mut() = Some(fut.await); // set out output value
        });

        loop {
            let mut fut = self.tasks.borrow_mut().pop_front().unwrap(); // take the first task

            if fut.as_mut().poll().is_pending() {
                self.tasks.borrow_mut().push_back(fut); // we need to re-queue our task now!
            }

            // exit our loop if we have our final value :)
            if let Some(output) = output.borrow_mut().take() {
                break output
            }
        }
    }
}

fn main() {
    let executor = Executor::default();
    executor.block_on(start());
}

async fn start() {
    // our magic business logic goes here
    println!("start!");
    for i in 0..10 {
        spawn(async move {
            println!("hello from task {i}");
        });
    }
    println!("spawned 10 tasks!");
}
