
use std::collections::HashMap;

use color_reduction::{reduce_colors, image::{ImageBuffer, Rgb}};
use image::{GenericImageView, DynamicImage};

fn main() {
    let color_path = "images/color.png";
    let material_path = "images/mat.png";
    //let height_path = "images/height.png";
    //let color = image::open("color.png").unwrap();
    let mat = image::open(material_path).unwrap();
    //let height = image::open(height_path).unwrap();
    let color_img = image::open(color_path).unwrap();
    
    let mut vox = create_vox::VoxFile::new(mat.height() as u16, mat.width() as u16, 1);
    //hashmap<(color_mat, color), index>
    //hashmap<color_mat, index>
    //iterate through all pixls in image
    
    let mut mat_index: HashMap<[u8;4], (i32, i32)> = HashMap::new();
    /*
    hard masonry:177-184  [0,128,128,255]
    hard metal:169-176    [128,128,0,255]
    plastic:153-168       [0,0,128,255]
    heavy metal:137-152   [0,128,0,255]
    weak metal:121-136    [128,0,0,255]
    plaster:105-120       [0,0,255,255]
    brick:89-104          [0,255,255,255]
    concrete:73-88        [255,0,255,255]
    wood:57-72            [255,255,0,255]
    rock:41-56            [128,128,128,255]
    dirt:25-40            [255,0,0,255]
    grass:9-24            [0,255,0,255]
    glass:1-8             [255,255,255,255]
     */
    mat_index.insert([255,255,255,255], (1, 8));
    mat_index.insert([0,255,0,255], (9, 16));
    mat_index.insert([255,0,0,255], (25, 16));
    mat_index.insert([128,128,128,255], (41, 16));
    mat_index.insert([255,255,0,255], (57, 16));
    mat_index.insert([255,0,255,255], (73, 16));
    mat_index.insert([0,255,255,255], (89, 16));
    mat_index.insert([0,0,255,255], (105, 16));
    mat_index.insert([128,0,0,255], (121, 16));
    mat_index.insert([0,128,0,255], (137, 16));
    mat_index.insert([0,0,128,255], (153, 16));
    mat_index.insert([128,128,0,255], (169, 8));
    mat_index.insert([0,128,128,255], (177, 16));
    for i in mat_index.clone(){
        let color = reduce_material(i.0, i.1.1, color_path, &mat, &color_img);
        let mut used_colors: HashMap<Rgb<u8>, i32> = HashMap::new();
        let mut cur_index = i.1.0;
        for x in 0..color.width(){
            for y in 0..color.height(){
                let mat_pixel = mat.get_pixel(x, y);
                let col_pixel = *color.get_pixel(x, y);
                //let height = height.get_pixel(x, y).0[0];
                if mat_pixel.0 == i.0 {
                    if used_colors.contains_key(&col_pixel) {
                        vox.models[0].add_voxel_at_pos(x as u8, y as u8, 0, used_colors[&col_pixel] as u8).unwrap();
                    }else {
                        vox.set_palette_color(cur_index as u8, col_pixel.0[0], col_pixel.0[1], col_pixel.0[2], 255);
                        used_colors.insert(col_pixel, cur_index);
                        cur_index += 1;
                        vox.models[0].add_voxel_at_pos(x as u8, y as u8, 0, used_colors[&col_pixel] as u8).unwrap();
                    }
                }
            }


        }
    }
    vox.save("result.vox");

}

fn reduce_material(material_color: [u8; 4], max_colors: i32, file: &str, material_img: &DynamicImage, color_img: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    let mut all_colors = get_num_colors(material_color, material_img, color_img);
    all_colors.truncate(max_colors as usize);
    let image = color_reduction::image::open(file)
    .expect("error loading image");
    let new_image = reduce_colors(image, convert_to_rgb(all_colors).as_slice());
    return new_image
}

//returns sorted (according to how many) number of each color pixel for a cetain color in the matetrial image
// -> vec<color, amount>
fn get_num_colors(material_color: [u8;4], materials: &DynamicImage, color_img: &DynamicImage) -> Vec<([u8;4], i32)>{
    let mut color_count: Vec<([u8;4], i32)> = vec![];
    let mut detail = 120;
    while color_count.len() < 16 && (detail>20){
        for pixel in materials.pixels() {
            // checks if material of pixel is glass
            if pixel.2.0 == material_color {
                //might need to minus 1
                let mut contained = false;
                for i in 0..color_count.len() {
                    // executes when color count already contains color
                    if are_similar(color_count[i].0,color_img.get_pixel(pixel.0, pixel.1).0, detail){
                        color_count[i].1 += 1;
                        contained = true
                    }
                }
                if !contained {
                    color_count.push((color_img.get_pixel(pixel.0, pixel.1).0,1))
                }
            }
        }
        detail -= 10
    }
    //progressively adds more similar colors to the palette
    
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
fn are_similar(color1: [u8;4], color2: [u8;4], sensitivity: i32) -> bool{
    (
        color1[0].abs_diff(color2[0]) +
        color1[1].abs_diff(color2[1]) +
        color1[2].abs_diff(color2[2])
    ) < sensitivity as u8
    //higher = to less colors in final image
}