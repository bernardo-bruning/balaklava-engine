pub mod gfx;

pub trait Graphics {

}

pub trait Backend <G: Graphics> {
    fn graphics() -> G;
}