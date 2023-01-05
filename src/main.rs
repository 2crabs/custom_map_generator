use color_reduction::reduce_colors;
use create_vox::Color;
use image::{GenericImageView, Rgba};

fn main() {
    
    for color in get_num_colors([255,255,255,255]){
        println!("{:?}", color);
    }
    

    //reduce colors
    reduce("color.png", get_common_colors());
    //end  reduction

    let img = image::open("color.png").unwrap();
    
    let mut vox = create_vox::VoxFile::new(128, 128, 1);
    let mut cur_palette_index = 0;

    //iterate through all pixls in image
    for pixel in img.pixels(){
        //add color to palette if not already there
        let rgba = pixel.2;
        let color = Color::new(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
        let mut is_in_palette = false;
        //iterate through each palette index
        for index in 1..255{
            if color == vox.palette[index-1] {
                is_in_palette = true;
                vox.models[0].add_voxel_at_pos(pixel.0 as u8, pixel.1 as u8, 0, index as u8).unwrap();
                break;
            }
        }
        //add color to palette
        if !is_in_palette{
            vox.palette[cur_palette_index] = color;
            cur_palette_index += 1;
            vox.models[0].add_voxel_at_pos(pixel.0 as u8, pixel.1 as u8, 0, cur_palette_index as u8).unwrap();
        }
    }
    vox.save("result.vox")

}

fn reduce(file: &str, colors: Vec<color_reduction::image::Rgb<u8>>){
    let image = color_reduction::image::open(file)
    .expect("error loading image");
    let new_image = reduce_colors(image, colors.as_slice());
    new_image.save(file).expect("error saving image");
}

fn get_common_colors() -> Vec<color_reduction::image::Rgb<u8>>{
    let mut colors: Vec<color_reduction::image::Rgb<u8>> =  vec![];
    /*
    hard masonry:177-184
    hard metal:169-176
    plastic:153-168
    heavy metal:137-152
    weak metal:121-136
    plaster:105-120
    brick:89-104
    concrete:73-88
    wood:57-72
    rock:41-56
    dirt:25-40
    grass:9-24
    glass:1-8
     */
    //need to reduce materials before using
    //first argument is which color is considered that material
    let mut glass_colors = get_num_colors([255,255,255,255]);
    glass_colors.truncate(8);
    colors.append(& mut convert_to_rgb(glass_colors));

    let mut grass_colors = get_num_colors([0,255,0,255]);
    grass_colors.truncate(16);
    colors.append(& mut convert_to_rgb(grass_colors));

    let mut dirt_colors = get_num_colors([255,0,0,255]);
    dirt_colors.truncate(16);
    colors.append(& mut convert_to_rgb(dirt_colors));

    let mut rock_colors = get_num_colors([128,128,128,255]);
    rock_colors.truncate(16);
    colors.append(& mut convert_to_rgb(rock_colors));

    let mut wood_colors = get_num_colors([255,255,0,255]);
    wood_colors.truncate(16);
    colors.append(& mut convert_to_rgb(wood_colors));

    let mut concrete_colors = get_num_colors([255,0,255,255]);
    concrete_colors.truncate(16);
    colors.append(& mut convert_to_rgb(concrete_colors));

    let mut brick_colors = get_num_colors([0,255,255,255]);
    brick_colors.truncate(16);
    colors.append(& mut convert_to_rgb(brick_colors));

    let mut plaster_colors = get_num_colors([0,0,255,255]);
    plaster_colors.truncate(16);
    colors.append(& mut convert_to_rgb(plaster_colors));

    let mut weak_metal_colors = get_num_colors([255,255,255,255]);
    weak_metal_colors.truncate(16);
    colors.append(& mut convert_to_rgb(weak_metal_colors));

    let mut heavy_metal_colors = get_num_colors([255,255,255,255]);
    heavy_metal_colors.truncate(16);
    colors.append(& mut convert_to_rgb(heavy_metal_colors));

    let mut plastic_colors = get_num_colors([255,255,255,255]);
    plastic_colors.truncate(16);
    colors.append(& mut convert_to_rgb(plastic_colors));

    let mut hard_metal_colors = get_num_colors([255,255,255,255]);
    hard_metal_colors.truncate(8);
    colors.append(& mut convert_to_rgb(hard_metal_colors));

    let mut hard_masonry_colors = get_num_colors([255,255,255,255]);
    hard_masonry_colors.truncate(8);
    colors.append(& mut convert_to_rgb(hard_masonry_colors));

    return colors
}

//returns sorted (according to how many) number of each color pixel for a cetain color in the matetrial image
fn get_num_colors(material_color: [u8;4]) -> Vec<([u8;4], i32)>{
    let materials = image::open("mat.png").unwrap();
    let color_img = image::open("color.png").unwrap();
    let mut color_count: Vec<([u8;4], i32)> = vec![];
    for pixel in materials.pixels() {
        // checks if material of pixel is glass
        if pixel.2.0 == material_color {
            //might need to minus 1
            let mut contained = false;
            for i in 0..color_count.len() {
                // executes when color count already contains color
                if color_count[i].0 == color_img.get_pixel(pixel.0, pixel.1).0{
                    color_count[i].1 += 1;
                    contained = true
                }
            }
            if !contained {
                color_count.push((color_img.get_pixel(pixel.0, pixel.1).0,1))
            }
        }
    }
    color_count.sort_by(|a, b| b.1.cmp(&a.1));
    return color_count;
}
fn convert_to_rgb(colors_in: Vec<([u8;4], i32)>) -> Vec<color_reduction::image::Rgb<u8>>{
    let mut new_colors: Vec<color_reduction::image::Rgb<u8>> = vec![];
    for i in colors_in{
        new_colors.push(color_reduction::image::Rgb::from([i.0[0],i.0[1],i.0[2]]))
    }
    return new_colors
}
