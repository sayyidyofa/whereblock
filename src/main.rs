use fastanvil::{CurrentJavaChunk, Region};
use fastnbt::from_bytes;
use std::fs::read_dir;

fn get_absolute_coords(chunk_x: i32, chunk_z: i32, block_x: i32, block_y: i32, block_z: i32) -> (i32, i32, i32) {
    let chunk_size = 16;
    let world_x = chunk_x * chunk_size + block_x;
    let world_y = block_y;
    let world_z = chunk_z * chunk_size + block_z;
    (world_x, world_y, world_z)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let pattern = args[1].clone();
    let regions_dir_path = args[2].clone();

    let region_mca_paths = read_dir(regions_dir_path.clone());

    if region_mca_paths.is_err() {
        eprintln!("Error when listing mca files in path {0}", regions_dir_path.clone());
        return;
    }

    let unwrapped_region_mca_paths = region_mca_paths.unwrap();

    unwrapped_region_mca_paths.for_each(|path| {
        let current_mca_file = std::fs::File::open(path.as_ref().unwrap().path()).unwrap();

        let mut region = Region::from_stream(current_mca_file).unwrap();

        (0..32).into_iter().for_each(|chunk_x| {
            (0..32).into_iter().for_each(|chunk_z| {
                let data = match region.read_chunk(chunk_x, chunk_z) {
                    Ok(Some(data)) => data,
                    Ok(None) => {
                        return;
                    },
                    Err(e) => {
                        eprintln!("Error while reading chunk (x: {0}, y: {1}) at file {2} -> {3}", chunk_x.to_string(), chunk_z.to_string(), path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap(), e.to_string());
                        return;
                    }
                };

                let chunk: CurrentJavaChunk = from_bytes(data.as_slice()).unwrap();

                chunk.sections.unwrap().sections().into_iter().for_each(|section| {
                    (0..16).into_iter().for_each(|x| {
                        (0..16).into_iter().for_each(|y| {
                            (0..16).into_iter().for_each(|z| {
                                let block_name = section.block_states.at(x, y, z).unwrap().name();
                                if block_name.to_string().contains(pattern.as_str()) {
                                    println!("Block at mca {0} chunk {1}, {2} pos x: {3}, y: {4}, z: {5} has name: {6} AND description: {7}",
                                        path.as_ref().unwrap().file_name().into_string().unwrap(),
                                        chunk_x,
                                        chunk_z,
                                        x, y, z,
                                        block_name,
                                        section.block_states.at(x, y, z).unwrap().encoded_description()
                                    );
                                }
                            });
                        });
                    });
                });
            });
        });
    });
}
