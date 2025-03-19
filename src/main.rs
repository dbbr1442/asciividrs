use core::time;
use std::env;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::thread;
use std::usize;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::fs::OpenOptions;
use chrono::Utc;
use rodio::{Decoder, OutputStream, source::Source};
use std::io::BufReader;
use std::fs;


fn slideshower(args: Vec<String>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("/home/lucy/Documents/asciividrs/bad-apple-player/src/bad_apple2.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    let mut file_iterator: i64 = 2;

    let start_time = Utc::now().time();
    //sl.play(&wav);
    let _ = stream_handle.play_raw(source.convert_samples());
    while file_iterator < (args.len()-2).try_into().unwrap() {
        let file_path = Path::new(&args[file_iterator as usize]);
        //let file = File::open(file_path);
        let file_data = fs::read(file_path).expect("error: cant read slide");
        for element in file_data.iter() {
            print!("{}", *element as char);
        }
        //println!("{}", file_data);

        let time_since = Utc::now().time() - start_time;
        let time_since: f64 = time_since.num_milliseconds() as f64/1000.0;
        let rounded_time_since: f64 = time_since*30.0;
        let rounded_time_since = rounded_time_since.round() as i64;
        println!("{} : {} : {}", file_iterator, rounded_time_since, file_iterator-(rounded_time_since));
        file_iterator = file_iterator-(file_iterator-(rounded_time_since));

        thread::sleep(time::Duration::from_micros(20000));
        print!("{}[2J", 27 as char);
        //print!("\x1B[2J\x1B[1;1H");

    }



    return;
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args[1] == "slideshow" {
        slideshower(args);
        return;
    }
    let ascii_art = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let ascii_art_array: Vec<char> = ascii_art.chars().collect();
    let mut iterator: u32 = 0;
    for file in args.iter().skip(1) {
        let mut file_path_new = String::from("/home/lucy/Documents/asciividrs/bad-apple-player/src/imgs_ascii/frame_0000");
        let iterator_string = iterator.to_string();
        file_path_new.push_str(&iterator_string);
        //let data: Vec<u8> = fs::read(file)?;
        let mut file = match File::open(Path::new(file)) {
            Err(why) => panic!("couldnt open: {}", why),
            Ok(file) => file,
        };
        println!("file: {:?}", file);

        let mut buf2 = [0u8; 2];
        file.read_exact_at(&mut buf2, 0).expect("error: reading signature");
        let signature: String = String::from_utf8(buf2.to_vec()).expect("error: converting signature");

        assert_eq!(signature, "BM");
        println!("  signature: {}", signature);

        let mut buf4 = [0u8; 4];

        file.read_exact_at(&mut buf2, 0x1C).expect("error: reading bpp");
        let bpp: u16 = u16::from_le_bytes(buf2);
        assert_eq!(bpp, 8);

        file.read_exact_at(&mut buf4, 0x1E).expect("error: reading compression");
        let compression: u32 = u32::from_le_bytes(buf4);
        assert_eq!(compression, 0);

        let mut buf1 = [0u8; 1];

        file.read_exact_at(&mut buf4, 0xA).expect("error: reading data_offset");
        let data_offset: u32 = u32::from_le_bytes(buf4);

        file.read_exact_at(&mut buf4, 0x12).expect("error: reading width");
        let width: u32 = u32::from_le_bytes(buf4);

        file.read_exact_at(&mut buf4, 0x16).expect("error: reading height");
        let height: u32 = u32::from_le_bytes(buf4);

        println!("  width: {}", width);
        println!("  height: {}", height);
        println!("  compression: 0");
        println!("  bpp: 8");
        println!("  data_offset: {}", data_offset);

        //let mut image = [[' '; width]; height];
        let mut image: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];
        file.seek(SeekFrom::Start(data_offset as u64)).expect("error: unable to seek");

        for line in (1..height).rev() {
            let mut line_vector = vec![' '; width as usize];
            for column in 0..width {
                 file.read(&mut buf1).expect("error: reading pixel");
                 let pixel_value: u8 = u8::from_le_bytes(buf1);
                 //print!("{}, ", pixel_value);
                 let ascii_art_index = 0.2734375*pixel_value as f64;
                 let pixel_ascii: char = ascii_art_array[ascii_art_index.floor() as u16 as usize];
                 //print!("{}", pixel_ascii);
                 //print!("{}", pixel_ascii);
                 line_vector[column as usize] = pixel_ascii;
            }
            image[(line-1) as usize] = line_vector;
            //println!();
        }
        let output_file_path = Path::new(&file_path_new);
        //if output_file_path.exists() {
          //  fs::remove_file(output_file_path).expect("error: removing preexisting file");
        //}

        //let mut output_file = File::create(output_file_path).expect("error: opening output_file");
        let mut output_file = OpenOptions::new().write(true).create(true).open(output_file_path).expect("error: opening output_file"); 

        for line in image.iter() {
            for pixel in line.iter() {
                write!(output_file, "{}", pixel).expect("error: writing to ofile");
                write!(output_file, "{}", pixel).expect("error: writing to ofile");
                print!("{}", pixel);
                print!("{}", pixel);
            }
            writeln!(output_file).expect("error: writing to ofile");
            println!();
        }
        iterator = iterator + 1;
    }
}
