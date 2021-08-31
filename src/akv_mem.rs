use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windowsF"))]
const USAGE: &str = "
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE).as_ref();
    let key = args.get(3).expect(USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(fname); 
    let mut store = ActionKV::open(path).expect("Unable to open file");
    store.load().expect("Unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?}", key),
            Some(value) => println!("{:?}", value) 
        }
        _ => eprintln!("{}", &USAGE)
    }
}
