use super::*;

#[derive(Deref)]
pub struct Texture {
    #[deref]
    inner: ugli::Texture,
}

impl geng::LoadAsset for Texture {
    fn load(geng: &Rc<Geng>, path: &str) -> geng::AssetFuture<Self> {
        let geng = geng.clone();
        <ugli::Texture as geng::LoadAsset>::load(&geng, path)
            .map(move |data| {
                let mut data = data?;
                data.set_filter(ugli::Filter::Nearest);
                Ok(Self { inner: data })
            })
            .boxed_local()
    }

    const DEFAULT_EXT: Option<&'static str> = Some("png");
}

#[derive(Deref)]
pub struct Font {
    #[deref]
    inner: Rc<geng::Font>,
}

impl geng::LoadAsset for Font {
    fn load(geng: &Rc<Geng>, path: &str) -> geng::AssetFuture<Self> {
        let geng = geng.clone();
        <Vec<u8> as geng::LoadAsset>::load(&geng, path)
            .map(move |data| {
                Ok(Font {
                    inner: Rc::new(geng::Font::new(&geng, data?)?),
                })
            })
            .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("ttf");
}

impl geng::LoadAsset for Level {
    fn load(geng: &Rc<Geng>, path: &str) -> geng::AssetFuture<Self> {
        Box::pin(
            <String as geng::LoadAsset>::load(geng, path).map(|s| Ok(serde_json::from_str(&s?)?)),
        )
    }
    const DEFAULT_EXT: Option<&'static str> = None;
}

#[derive(geng::Assets)]
pub struct Assets {
    pub cat: Texture,
    pub mouse: Texture,
    pub dog: Texture,
    pub grass: Texture,
    pub bush: Texture,
    #[asset(path = "flower*.png", range = "1..=3")]
    pub flower: Vec<Texture>,
    pub bone: Texture,
    #[asset(path = "box.png")]
    pub box_asset: Texture,
    pub cheese: Texture,
    pub doghouse: Texture,
    pub fence: Texture,
    pub wall: Texture,
    pub water: Texture,
    pub fish: Texture,
    #[asset(path = "levels/level*.json", range = "1..=8")]
    pub levels: Vec<Level>,
    pub font: Texture,
}

impl Assets {
    pub fn entity(&self, entity: EntityType) -> &ugli::Texture {
        match entity {
            EntityType::Bush => &self.bush,
            EntityType::Doghouse => &self.doghouse,
            EntityType::Cat => &self.cat,
            EntityType::Dog => &self.dog,
            EntityType::Mouse => &self.mouse,
            EntityType::Box => &self.box_asset,
            EntityType::Cheese => &self.cheese,
            EntityType::Fence => &self.fence,
            EntityType::Wall => &self.wall,
            EntityType::Water => &self.water,
            EntityType::Bone => &self.bone,
            EntityType::Fish => &self.fish,
        }
    }
}
