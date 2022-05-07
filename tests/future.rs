#![allow(dead_code)]
#![allow(unused_variables)]

// 利用标准库实现自定义Future类型

use std::{task::Waker, sync::{Arc}, time::Duration, future::Future};

use tokio::sync::Mutex;


struct SharedState {
    completed: bool,
    waker: Option<Waker>
}

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}


// 这个mutex是不可重入的
#[test]
fn rein_mutex() {
    let data = Arc::new(std::sync::Mutex::new(0));
    let mut d1 = data.lock().unwrap();
    *d1 += 1;
    // drop(d1);
    let mut d2 = data.lock().unwrap();
    *d2 += 2;
    // drop(d2);

    println!("the value: {:#?}", data);
}

/// std::sync::Mutex 在tokio种使用必须不跨过await, 否则不能通过编译
/// drop之类的转移所有权函数还不如加个大括号有用
#[test]
fn tokio_deadlock() {
    // let data = Arc::new(Mutex::new(0));
    let data = Arc::new(std::sync::Mutex::new(0));
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build().unwrap();
    let data1 = data.clone();
    let data2 = data.clone();
    let jh1 = rt.spawn(async move {
        println!("before d1 locked");
        // let mut d1 = data1.lock().await;
        {
            let mut d1 = data1.lock().unwrap();
            println!("after d1 locked");
            *d1 += 1;
            // drop(d1);
        }
        println!("before task1 sleep");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("after task1 sleep");
    });
    let jh2 = rt.spawn(async move {
        println!("before d2 locked");
        // let mut d2 = data2.lock().await;
        {
            let mut d2 = data2.lock().unwrap();
            println!("after d2 locked");
            *d2 += 2;
            // drop(d2);
        }
        println!("before task2 sleep");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("after task2 sleep");
    });

    rt.block_on(async move {
        let _ = jh1.await;
        let _ = jh2.await;
    });
    println!("main process after join");
}

// select时, 所有分支future都会执行(await)
#[test]
fn select_test() {
    fn gen_future(i: i32) -> impl Future<Output = ()> {
        async move {
            println!("before future{i} sleep");
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("after future{i} sleep");
        }
    }
    let f1 = gen_future(1);
    let f2 = gen_future(2);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async move {
        tokio::select! {
            _ = f1 => {
                println!("future1 selected");
            }
            _ = f2 => {
                println!("future2 selected");
            }
        }
    })
}
