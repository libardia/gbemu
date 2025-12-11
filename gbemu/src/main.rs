use log::info;

#[allow(dead_code)]
fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    gbemu_hardware::cpu::test();
    gbemu_hardware::mmu::test();
    info!("main()");
}
