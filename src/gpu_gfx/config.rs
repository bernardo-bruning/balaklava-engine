pub struct Config {
    pub title: String,
    pub dimension_width: usize,
    pub dimension_height: usize,
    pub fullscreen: bool
}

impl Config {
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_dimensions(mut self, width: usize, height: usize) -> Self {
        self.dimension_width = width;
        self.dimension_height = height;
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }
}