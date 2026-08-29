fn main() {
    // loading the image in
    let file_path = "obraz2.jpg";
    let img =image::open(file_path).unwrap();
    
    // rgb to luma conversion
    let img_luma = img.to_luma8();
    
    // array holding light levels , 16 fields for brightness levels (normalized)
    let mut  brightness =[0u64;16];

    // building histogram
    for pixel in img_luma.pixels() {
         brightness[pixel[0] as usize /16] +=1;
    }
    // finding the  highiest count of the pixels with same brightness
    let max = * brightness.iter().max().unwrap_or(&1);

    // printing out the histogram and legend 
    println!("BRIGHTNESS HISTOGRAM");
    println!("File: {file_path} ({0}x{1} px)", img.width(), img.height());
    println!("Legend: \nY-axis = Pixel Count (Normalized 0-10) , \nX-axis = Brightness (0=Dark (brightnes from 0 to 15) -> 15=Bright (brightness from 240 to 255))");
    println!();

    // Rendering vertical Y-axis (10 height levels)
    for level in (1..=10).rev() {
        print!("{:2} | ", level);
        for &count in & brightness {
            let bar_height = (count as usize * 10) / max as usize;
            if bar_height >= level {
                print!(" # ");
            } else {
                print!("   ");
            }
        }
        println!();
    }

    // Rendering  X-axis line (scale from 0 to 15 )
    println!("   +{}", "-".repeat(16 * 3));
    print!("     ");
    for i in 0..16 {
        print!("{:2} ", i);
    }
    println!("\n     (Dark                              Bright)");
}