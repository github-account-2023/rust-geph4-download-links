use geph4_download_links::get;

fn main() {
    get();
    std::thread::sleep(std::time::Duration::from_secs(30));
}
