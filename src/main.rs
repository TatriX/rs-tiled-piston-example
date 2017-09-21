extern crate piston_window;
extern crate find_folder;
extern crate tiled;

use piston_window::*;
use std::fs::File;
use tiled::parse;

fn main() {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let file = File::open(assets.join("tiled_base64_zlib.tmx")).unwrap();
    let map = parse(file).unwrap();

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: tiled", [600, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let tileset = map.get_tileset_by_gid(1).unwrap();
    let tile_width = tileset.tile_width;
    let tile_height = tileset.tile_height;

    let tilesheet = assets.join(&tileset.images[0].source);
    let tilesheet = Texture::from_path(
        &mut window.factory,
        &tilesheet,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let (width, _) = tilesheet.get_size();
    let layer: &tiled::Layer = &map.layers[0];
    let image = Image::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5; 4], g);

            for (y, row) in layer.tiles.iter().enumerate().clone() {
                for (x, &tile) in row.iter().enumerate() {
                    if tile == 0 {
                        continue;
                    }

                    let tile = tile - 1;
                    let x = x as u32 * tile_width;
                    let y = y as u32 * tile_height;

                    let st = DrawState::default().scissor([x, y, tile_width, tile_height]);
                    let dx = tile % (width / tile_width);
                    let dy = tile / (width / tile_height);

                    let trans = c.transform
                        .trans(-((dx * tile_width) as f64), -((dy * tile_height) as f64))
                        .trans(x as f64, y as f64);

                    image.draw(&tilesheet, &st, trans, g);
                }
            }
        });
    }

}
