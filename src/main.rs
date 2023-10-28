use std::env;
use std::fs::File;
use std::io::Read;

const GLOBAL_BUFFER_LENGTH: usize = 16;

fn get_file(path_to_file: String) -> File{
    match File::open(path_to_file){
        Ok(f) => File::from(f),
        Err(_) => panic!("Invalid file path")
    }
}

fn get_hex_representation(byte_array: &mut [u8]) -> String{
    let string_vec: Vec<String> = byte_array.chunks(2).map(
        |chunk|{
            if chunk.len() == 2 {
                format!("{:02x}{:02x}", chunk[0], chunk[1])
            } else {
                format!("{:02x}", chunk[0])
            }
        }
    ).collect();
    string_vec.join(" ")
}

fn get_ascii_representation(byte_array: &mut [u8]) -> String{
    let string_vec: Vec<String> = byte_array.iter().map(
        |num|{
            if *num >= 32 && *num <= 126{
                (*num as char).to_string()
            } else {
                '.'.to_string()
            }
        }
    ).collect();
    string_vec.join("")
}
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2{
        panic!("Error! Please provide file to hexdump.\nExample: my_hexdump <file_name>");
    }

    let mut file_to_read: File = get_file(String::from(&args[1]));

    let mut buff: [u8; 16] = [0; GLOBAL_BUFFER_LENGTH];
    let mut offset: usize = 0;

    loop{
        let bytes_read = file_to_read.read(&mut buff);
        match bytes_read{
            Ok(number) => {
                if number == 0{
                    break ;
                } else {
                    println!("{:08x}: {:40} {:10}"
                        , offset
                        , get_hex_representation(&mut buff[0..number])
                        , get_ascii_representation(&mut buff[0..number]));
                    offset += GLOBAL_BUFFER_LENGTH;
                }
            },
            Err(err) => {
                eprintln!("my_hexdump: {}",err);
                break ;
            }
        }
    }
}
