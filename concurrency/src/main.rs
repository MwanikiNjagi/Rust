use std::thread;
use std::time::Duration;
//common problems with parallel programming
//1.Threads are accessing data in the wrong order
//2.Threads are blocked from executing because of confusion over requirements to proceed with execution

fn main() {
   let thread1 = thread::spawn(||{
    for i in 1..25{
        println!("Spawned thread: {}", i);
        thread::sleep(Duration::from_millis(1));

    }
   });

   for i in 1..20{
    println!("Main thread: {}", i);
    thread::sleep(Duration::from_millis(1));
   }

   //makes sure that both threads are able to complete
   thread1.join().unwrap();
}
