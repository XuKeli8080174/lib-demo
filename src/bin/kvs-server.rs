use kvs::thread_pool::{NaiveThreadPool, ThreadPool, RayonThreadPool, SharedQueueThreadPool};
use log::LevelFilter;
use log::{error, info, warn};
use std::env::current_dir;
use std::fs;
use std::net::SocketAddr;
use std::process::exit;
use std::str::FromStr;

use clap::{ArgEnum, Parser};

use kvs::{KvStore, KvsEngine, KvsServer, Result, SledKvsEngine};

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(Parser, Debug)]
#[clap(name = "kvs-server")]
#[clap(author, version, about, long_about = None)]
struct Opt {
    #[clap(
        long,
        help = "Sets the listening address",
        value_name = "IP:PORT",
        default_value = DEFAULT_LISTENING_ADDRESS,
        parse(try_from_str)
    )]
    addr: SocketAddr,
    #[clap(
        arg_enum,
        long,
        help = "Sets the storage engine",
        value_name = "ENGINE-NAME"
    )]
    engine: Option<Engine>,
    #[clap(
        arg_enum,
        long,
        help = "Sets the thread pool",
        value_name = "THREAD-POOL",
        default_value_t = Pool::native,
    )]
    pool: Pool,
}

#[allow(non_camel_case_types)]
#[derive(ArgEnum, Debug, Copy, Clone, PartialEq, Eq)]
enum Engine {
    kvs,
    sled,
}

#[allow(non_camel_case_types)]
#[derive(ArgEnum, Debug, Copy, Clone, PartialEq, Eq)]
enum Pool {
    native,
    rayon,
    shared_queue,
}

impl FromStr for Engine {
    type Err = String;

    fn from_str(input: &str) -> std::result::Result<Engine, Self::Err> {
        match input {
            "kvs" => Ok(Engine::kvs),
            "sled" => Ok(Engine::sled),
            str => Err(format!("engine: {} not exists", str)),
        }
    }
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let mut opt = Opt::parse();
    let res = current_engine().and_then(move |curr_engine| {
        if opt.engine.is_none() {
            opt.engine = curr_engine;
        }
        if curr_engine.is_some() && opt.engine != curr_engine {
            error!("Wrong engine!");
            exit(1);
        }
        run(opt)
    });
    if let Err(e) = res {
        error!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {:?}", engine);
    info!("Thread pool: {:?}", opt.pool);
    info!("Listening on {}", opt.addr);

    // write engine to engine file
    fs::write(current_dir()?.join("engine"), format!("{:?}", engine))?;

    let concurrency = num_cpus::get() as u32;
    match opt.pool {
        Pool::native => run_with::<NaiveThreadPool>(engine, &opt, concurrency),
        Pool::rayon => run_with::<RayonThreadPool>(engine, &opt, concurrency),
        Pool::shared_queue => run_with::<SharedQueueThreadPool>(engine, &opt, concurrency),
    }
}

fn run_with<P: ThreadPool>(engine: Engine, opt: &Opt, concurrency: u32) -> Result<()> {
    match engine {
        Engine::kvs => run_with_engine(KvStore::<P>::open(current_dir()?, concurrency)?, opt.addr),
        Engine::sled => run_with_engine(
            SledKvsEngine::<P>::new(sled::open(current_dir()?)?, concurrency)?,
            opt.addr,
        ),
    }
}

fn run_with_engine<E: KvsEngine>(
    engine: E,
    addr: SocketAddr,
) -> Result<()> {
    let server = KvsServer::new(engine);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        server.run(addr).await
    })
}

fn current_engine() -> Result<Option<Engine>> {
    let engine = current_dir()?.join("engine");
    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine)?.parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            warn!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }
}
