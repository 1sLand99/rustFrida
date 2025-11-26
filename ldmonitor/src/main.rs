use clap::Parser;
use ldmonitor::DlopenMonitor;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long)]
    pid: Option<u32>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let opt = Opt::parse();
    let monitor = DlopenMonitor::new(opt.pid)?;

    println!("Monitoring android_dlopen_ext calls...");
    println!("Press Ctrl-C to exit.");

    while let Some(info) = monitor.recv() {
        println!(
            "android_dlopen_ext called: pid={}, uid={}, path={}",
            info.pid, info.uid, info.path
        );
    }

    Ok(())
}
