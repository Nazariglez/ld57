use rkit::{
    assets::{AssetList, AssetMap},
    audio::{Sound, create_sound},
    draw::{Font, Sprite, create_font, create_sprite},
    gfx::{self, *},
    math::{Rect, Vec2, vec2},
    prelude::*,
};

use crate::consts::*;
use crate::screens::AppScreen;

const ASSETS_DIR: &str = "./assets";

pub fn init_assets(screen: AppScreen, app: &mut App) -> &mut App {
    app.add_systems(
        OnEnter(screen),
        init_assets_loader_system.run_if(can_init_load),
    )
    .add_screen_systems(
        screen,
        OnPreUpdate,
        update_assets_loader_system.run_if(can_load),
    )
}

// -- loader
#[derive(Resource, Deref)]
pub struct AssetLoader(pub AssetList);

impl AssetLoader {
    pub fn new() -> Result<Self, String> {
        let nearest_sampler = gfx::create_sampler()
            .with_label("Pixel Sampler")
            .with_min_filter(TextureFilter::Nearest)
            .with_mag_filter(TextureFilter::Nearest)
            .build()?;

        let list = AssetList::new(&[
            &data_dir("kenney_pixel-webfont.ttf"),
            &img_dir("spritesheet.png"),
        ])
        .with_extension_parser("png", move |id, data| {
            parse_sprite(id, data, &nearest_sampler)
        })
        .with_extension_parser("ogg", parse_ogg)
        .with_extension_parser("ttf", parse_font);

        Ok(Self(list))
    }
}

fn parse_sprite(_id: &str, data: &[u8], sampler: &Sampler) -> Result<Sprite, String> {
    create_sprite()
        .from_image(data)
        .with_sampler(sampler)
        .build()
}

fn parse_font(_id: &str, data: &[u8]) -> Result<Font, String> {
    create_font(data).with_nearest_filter(true).build()
}

fn parse_ogg(_id: &str, data: &[u8]) -> Result<Sound, String> {
    create_sound(data)
}

fn img_dir(d: &str) -> String {
    format!("{}/images/{}", ASSETS_DIR, d)
}

fn snd_dir(d: &str) -> String {
    format!("{}/snd/{}", ASSETS_DIR, d)
}

fn data_dir(d: &str) -> String {
    format!("{}/data/{}", ASSETS_DIR, d)
}

fn init_assets_loader_system(mut cmds: Commands) {
    let loader = AssetLoader::new().or_panic("Initiating AssetLoader");
    cmds.insert_resource(loader);
}

fn can_init_load(loader: Option<Res<AssetLoader>>, assets: Option<Res<Assets>>) -> bool {
    loader.is_none() && assets.is_none()
}

fn can_load(loader: Option<Res<AssetLoader>>, assets: Option<Res<Assets>>) -> bool {
    loader.is_some() && assets.is_none()
}

fn update_assets_loader_system(mut loader: ResMut<AssetLoader>, mut cmds: Commands) {
    let parsed = loader.parse(Assets::new).or_panic("Parsing Assets");
    if let Some(assets) = parsed {
        cmds.insert_resource(assets);
        cmds.remove_resource::<AssetLoader>();
    }
}

// -- assets
#[derive(Resource)]
pub struct Assets {
    pub font: Font,

    pub empty_square: Sprite,
    pub dotted_square: Sprite,
    pub white_square: Sprite,
    pub money: Sprite,
    pub cobber: Sprite,
    pub iron: Sprite,
    pub silver: Sprite,
    pub gold: Sprite,
    pub wood: Sprite,
    pub food: Sprite,
    pub people: Sprite,
    pub ring: Sprite,
    pub farm: Sprite,
    pub house: Sprite,
    pub forest: Sprite,
    pub factory: Sprite,
    pub shop: Sprite,
}

impl Assets {
    fn new(list: &AssetMap) -> Result<Self, String> {
        let font = list.get(&data_dir("kenney_pixel-webfont.ttf"))?;

        let tile_size = Vec2::splat(TILE_SIZE);
        let spritesheet = list.get::<Sprite>(&img_dir("spritesheet.png"))?;
        let row = 0.0;
        let empty_square = spritesheet.clone_with_frame(Rect::new(vec2(0.0, row), tile_size));
        let dotted_square =
            spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE, row), tile_size));
        let white_square =
            spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 2.0, row), tile_size));

        let row = TILE_SIZE * 2.0;
        let money = spritesheet.clone_with_frame(Rect::new(vec2(0.0, row), tile_size));
        let cobber = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE, row), tile_size));
        let iron = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 2.0, row), tile_size));
        let silver = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 3.0, row), tile_size));
        let gold = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 4.0, row), tile_size));
        let wood = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 5.0, row), tile_size));
        let food = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 6.0, row), tile_size));
        let people = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 7.0, row), tile_size));
        let ring = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 8.0, row), tile_size));

        let row = TILE_SIZE * 4.0;
        let farm = spritesheet.clone_with_frame(Rect::new(vec2(0.0, row), tile_size));
        let house = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE, row), tile_size));
        let forest = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 2.0, row), tile_size));
        let factory =
            spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 3.0, row), tile_size));
        let shop = spritesheet.clone_with_frame(Rect::new(vec2(TILE_SIZE * 4.0, row), tile_size));

        Ok(Self {
            font,
            empty_square,
            dotted_square,
            white_square,
            money,
            cobber,
            iron,
            silver,
            gold,
            wood,
            food,
            people,
            ring,
            farm,
            house,
            forest,
            factory,
            shop,
        })
    }
}
