#![allow(non_snake_case)]
#![allow(dead_code)]

use wasm_bindgen::prelude::*;

// import functions

#[wasm_bindgen]
extern
{
    fn wein2dWASM_sound_new(file_path: &str) -> u32;
    fn wein2dWASM_sound_setVolume(index: u32, volume: f32);
    fn wein2dWASM_sound_play(index: u32);
    fn wein2dWASM_sound_loop(index: u32, looping: bool);
    fn wein2dWASM_sound_stop(index: u32);
    fn wein2dWASM_sound_isPlaying(index: u32) -> bool;
    fn wein2dWASM_sound_drop(index: u32);

    fn wein2dWASM_sprite_new(file_path: &str) -> u32;
    fn wein2dWASM_sprite_getWidth(index: u32) -> u32;
    fn wein2dWASM_sprite_getHeight(index: u32) -> u32;
    fn wein2dWASM_sprite_drop(index: u32);

    fn wein2dWASM_virtualCanvas_new(width: u32, height: u32) -> u32;
    fn wein2dWASM_virtualCanvas_getWidth(index: u32) -> u32;
    fn wein2dWASM_virtualCanvas_getHeight(index: u32) -> u32;
    fn wein2dWASM_virtualCanvas_drawRect(destVirtualCanvasindex: u32, posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_virtualCanvas_drawOval(destVirtualCanvasindex: u32, posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_virtualCanvas_drawSprite(destVirtualCanvasindex: u32, spriteindex: u32, posX: f32, posY: f32, width: f32, height: f32, srcPosX: u32, srcPosY: u32, srcWidth: u32, srcHeight: u32, colorA: u8);
    fn wein2dWASM_virtualCanvas_drawText(destVirtualCanvasindex: u32, content: &str, posX: f32, posY: f32, positioning: &str, fontSize: f32, fontFamily: &str, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_virtualCanvas_fill(destVirtualCanvasindex: u32, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_virtualCanvas_drawLine(destVirtualCanvasindex: u32, posX: f32, posY: f32, endX: f32, endY: f32, lineWidth: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_virtualCanvas_drawVirtualCanvas(destVirtualCanvasindex: u32, virtualCanvasindex: u32, posX: f32, posY: f32, width: f32, height: f32, srcPosX: f32, srcPosY: f32, srcWidth: f32, srcHeight: f32, colorA: u8);
    fn wein2dWASM_virtualCanvas_drop(index: u32);

    fn wein2dWASM_getWidth() -> f32;
    fn wein2dWASM_getHeight() -> f32;
    fn wein2dWASM_getDeltaTime() -> f32;
    fn wein2dWASM_getKey(key: u32) -> bool;
    fn wein2dWASM_getTypedText() -> String;
    fn wein2dWASM_setTypedText(text: String);
    fn wein2dWASM_getMouseX() -> f32;
    fn wein2dWASM_getMouseY() -> f32;
    fn wein2dWASM_getMouseL() -> bool;
    fn wein2dWASM_getMouseS() -> bool;
    fn wein2dWASM_getMouseR() -> bool;
    fn wein2dWASM_drawRect(posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_drawOval(posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_drawSprite(spriteindex: u32, posX: f32, posY: f32, width: f32, height: f32, srcPosX: u32, srcPosY: u32, srcWidth: u32, srcHeight: u32, colorA: u8);
    fn wein2dWASM_drawText(content: &str, posX: f32, posY: f32, positioning: &str, fontSize: f32, fontFamily: &str, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_fill(colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_drawLine(posX: f32, posY: f32, endX: f32, endY: f32, lineWidth: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8);
    fn wein2dWASM_drawVirtualCanvas(virtualCanvasindex: u32, posX: f32, posY: f32, width: f32, height: f32, srcPosX: f32, srcPosY: f32, srcWidth: f32, srcHeight: f32, colorA: u8);
}


// textpositioning

pub struct TextPositioning {}

impl TextPositioning
{
    pub const LEFT: &'static str = "right";
    pub const CENTER: &'static str = "center";
    pub const RIGHT: &'static str = "left";
}

// key

pub struct Key {}

impl Key
{
    pub const CTRL: u32 = 0;
    pub const SHIFT: u32 = 1;
    pub const SPACE: u32 = 2;
    pub const BACKSPACE: u32 = 3;
    pub const ENTER: u32 = 4;
    pub const ALT: u32 = 5;
    pub const A: u32 = 6;
    pub const B: u32 = 7;
    pub const C: u32 = 8;
    pub const D: u32 = 9;
    pub const E: u32 = 10;
    pub const F: u32 = 11;
    pub const G: u32 = 12;
    pub const H: u32 = 13;
    pub const I: u32 = 14;
    pub const J: u32 = 15;
    pub const K: u32 = 16;
    pub const L: u32 = 17;
    pub const M: u32 = 18;
    pub const N: u32 = 19;
    pub const O: u32 = 20;
    pub const P: u32 = 21;
    pub const Q: u32 = 22;
    pub const R: u32 = 23;
    pub const S: u32 = 24;
    pub const T: u32 = 25;
    pub const U: u32 = 26;
    pub const V: u32 = 27;
    pub const W: u32 = 28;
    pub const X: u32 = 29;
    pub const Y: u32 = 30;
    pub const Z: u32 = 31;
    pub const UP: u32 = 32;
    pub const DOWN: u32 = 33;
    pub const LEFT: u32 = 34;
    pub const RIGHT: u32 = 35;
    pub const N0: u32 = 36;
    pub const N1: u32 = 37;
    pub const N2: u32 = 38;
    pub const N3: u32 = 39;
    pub const N4: u32 = 40;
    pub const N5: u32 = 41;
    pub const N6: u32 = 42;
    pub const N7: u32 = 43;
    pub const N8: u32 = 44;
    pub const N9: u32 = 45;
    pub const F1: u32 = 46;
    pub const F2: u32 = 47;
    pub const F3: u32 = 48;
    pub const F4: u32 = 49;
    pub const F5: u32 = 50;
    pub const F6: u32 = 51;
    pub const F7: u32 = 52;
    pub const F8: u32 = 53;
    pub const F9: u32 = 54;
    pub const F10: u32 = 55;
    pub const F11: u32 = 56;
    pub const F12: u32 = 57;
    pub const ESC: u32 = 58;
}

// boxcollision

pub fn rect_touching_rect(rect1PosX: f32, rect1PosY: f32, rect1SizeX: f32, rect1SizeY: f32, rect2PosX: f32, rect2PosY: f32, rect2SizeX: f32, rect2SizeY: f32) -> bool
{
    !(rect1PosX + rect1SizeX < rect2PosX) && !(rect2PosX + rect2SizeX < rect1PosX) && !(rect1PosY + rect1SizeY < rect2PosY) && !(rect2PosY + rect2SizeY < rect1PosY)
}

pub fn point_inside_rect(pointX: f32, pointY: f32, rectPosX: f32, rectPosY: f32, rectSizeX: f32, rectSizeY: f32) -> bool
{
    rectPosX < pointX && pointX < rectPosX + rectSizeX && rectPosY < pointY && pointY < rectPosY + rectSizeY
}

pub fn rect_inside_rect(rect1PosX: f32, rect1PosY: f32, rect1SizeX: f32, rect1SizeY: f32, rect2PosX: f32, rect2PosY: f32, rect2SizeX: f32, rect2SizeY: f32) -> bool
{
    rect2PosX < rect1PosX && rect1PosX + rect1SizeX < rect2PosX + rect2SizeX && rect2PosY < rect1PosY && rect1PosY + rect1SizeY < rect2PosY + rect2SizeY
}

// sound

pub struct Sound
{
    index: u32
}

impl Sound
{
    pub fn new(filePath: &str) -> Sound
    {
        Sound
        {
            index: wein2dWASM_sound_new(filePath)
        }
    }

    pub fn set_volume(&self, volume: f32)
    {
        wein2dWASM_sound_setVolume(self.index, volume);
    }

    pub fn play(&self) { wein2dWASM_sound_play(self.index); }

    pub fn set_looping(&self, looping: bool)
    {
        wein2dWASM_sound_loop(self.index, looping);
    }

    pub fn stop(&self)
    {
        wein2dWASM_sound_stop(self.index);
    }

    pub fn is_playing(&self) -> bool
    {
        wein2dWASM_sound_isPlaying(self.index)
    }
}

impl Drop for Sound
{
    fn drop(&mut self)
    {
        wein2dWASM_sound_drop(self.index);
    }
}

// sprite

pub struct Sprite
{
    index: u32
}

impl Sprite
{
    pub fn new(filePath: &str) -> Sprite
    {
        let spriteIndex = wein2dWASM_sprite_new(filePath);
        Sprite
        {
            index: spriteIndex
        }
    }

    pub fn get_width(&self) -> u32 { wein2dWASM_sprite_getWidth(self.index) }

    pub fn get_height(&self) -> u32 { wein2dWASM_sprite_getHeight(self.index) }
}

impl Drop for Sprite
{
    fn drop(&mut self)
    {
        wein2dWASM_sprite_drop(self.index);
    }
}

// virtualCanvas

pub struct VirtualCanvas
{
    index: u32,
    width: u32,
    height: u32
}

impl VirtualCanvas
{
    pub fn new(width: u32, height: u32) -> VirtualCanvas
    {
        let virtualCanvasIndex = wein2dWASM_virtualCanvas_new(width, height);
        VirtualCanvas
        {
            index: virtualCanvasIndex,
            width,
            height
        }
    }

    pub fn get_width(&self) -> u32
    {
        self.width
    }

    pub fn get_height(&self) -> u32
    {
        self.height
    }

    pub fn draw_rect(&self, posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
    {
        wein2dWASM_virtualCanvas_drawRect(self.index, posX, posY, width, height, colorA, colorR, colorG, colorB);
    }

    pub fn draw_oval(&self, posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
    {
        wein2dWASM_virtualCanvas_drawOval(self.index, posX, posY, width, height, colorA, colorR, colorG, colorB);
    }

    pub fn draw_sprite(&self, sprite: &Sprite, posX: f32, posY: f32, colorA: u8)
    {
        wein2dWASM_virtualCanvas_drawSprite(self.index, sprite.index, posX, posY, sprite.get_width() as f32, sprite.get_height() as f32, 0, 0, sprite.get_width(), sprite.get_height(), colorA);
    }

    pub fn draw_sprite_sized(&self, sprite: &Sprite, posX: f32, posY: f32, width: f32, height: f32, colorA: u8)
    {
        wein2dWASM_virtualCanvas_drawSprite(self.index, sprite.index, posX, posY, width, height, 0, 0, sprite.get_width(), sprite.get_height(), colorA);
    }

    pub fn draw_sprite_sized_source(&self, sprite: &Sprite, posX: f32, posY: f32, width: f32, height: f32, srcPosX: u32, srcPosY: u32, srcWidth: u32, srcHeight: u32, colorA: u8)
    {
        wein2dWASM_virtualCanvas_drawSprite(self.index, sprite.index, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA);
    }

    pub fn draw_text(&self, content: &str, posX: f32, posY: f32, fontSize: f32, fontFamily: &str, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
    {
        wein2dWASM_virtualCanvas_drawText(self.index, content, posX, posY, TextPositioning::RIGHT, fontSize, fontFamily, colorA, colorR, colorG, colorB);
    }

    pub fn draw_text_positioned(&self, content: &str, posX: f32, posY: f32, positioning: &str, fontSize: f32, fontFamily: &str, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
    {
        wein2dWASM_virtualCanvas_drawText(self.index, content, posX, posY, positioning, fontSize, fontFamily, colorA, colorR, colorG, colorB);
    }

    pub fn fill(&self, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
    {
        wein2dWASM_virtualCanvas_fill(self.index, colorA, colorR, colorG, colorB);
    }

    pub fn draw_line(&self, posX: f32, posY: f32, endX: f32, endY: f32, lineWidth: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
    {
        wein2dWASM_virtualCanvas_drawLine(self.index, posX, posY, endX, endY, lineWidth, colorA, colorR, colorG, colorB);
    }

    pub fn draw_virtual_canvas(&self, virtualCanvas: &VirtualCanvas, posX: f32, posY: f32, colorA: u8)
    {
        wein2dWASM_virtualCanvas_drawVirtualCanvas(self.index, virtualCanvas.index, posX, posY, virtualCanvas.width as f32, virtualCanvas.height as f32, 0f32, 0f32, virtualCanvas.width as f32, virtualCanvas.height as f32, colorA);
    }

    pub fn draw_virtual_canvas_sized(&self, virtualCanvas: &VirtualCanvas, posX: f32, posY: f32, width: f32, height: f32, colorA: u8)
    {
        wein2dWASM_virtualCanvas_drawVirtualCanvas(self.index, virtualCanvas.index, posX, posY, width, height, 0f32, 0f32, virtualCanvas.width as f32, virtualCanvas.height as f32, colorA);
    }

    pub fn draw_virtual_canvas_sized_source(&self, virtualCanvas: &VirtualCanvas, posX: f32, posY: f32, width: f32, height: f32, srcPosX: f32, srcPosY: f32, srcWidth: f32, srcHeight: f32, colorA: u8)
    {
        wein2dWASM_virtualCanvas_drawVirtualCanvas(self.index, virtualCanvas.index, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA);
    }
}

impl Drop for VirtualCanvas
{
    fn drop(&mut self)
    {
        wein2dWASM_virtualCanvas_drop(self.index);
    }
}

// wein2dapplication

pub fn get_width() -> f32 { wein2dWASM_getWidth() }

pub fn get_height() -> f32 { wein2dWASM_getHeight() }

pub fn get_delta_time() -> f32 { wein2dWASM_getDeltaTime() }

pub fn get_key(key: u32) -> bool { wein2dWASM_getKey(key) }

pub fn get_typed_text() -> String { wein2dWASM_getTypedText() }

pub fn set_typed_text(text: &str) { wein2dWASM_setTypedText(text.to_owned()); }

pub fn get_mouse_x() -> f32 { wein2dWASM_getMouseX() }

pub fn get_mouse_y() -> f32 { wein2dWASM_getMouseY() }

pub fn get_mouse_l() -> bool { wein2dWASM_getMouseL() }

pub fn get_mouse_s() -> bool { wein2dWASM_getMouseS() }

pub fn get_mouse_r() -> bool { wein2dWASM_getMouseR() }

pub fn draw_rect(posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
{
    wein2dWASM_drawRect(posX, posY, width, height, colorA, colorR, colorG, colorB);
}

pub fn draw_oval(posX: f32, posY: f32, width: f32, height: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
{
    wein2dWASM_drawOval(posX, posY, width, height, colorA, colorR, colorG, colorB);
}

pub fn draw_sprite(sprite: &Sprite, posX: f32, posY: f32, colorA: u8)
{
    wein2dWASM_drawSprite(sprite.index, posX, posY, sprite.get_width() as f32, sprite.get_height() as f32, 0, 0, sprite.get_width(), sprite.get_height(), colorA);
}

pub fn draw_sprite_sized(sprite: &Sprite, posX: f32, posY: f32, width: f32, height: f32, colorA: u8)
{
    wein2dWASM_drawSprite(sprite.index, posX, posY, width, height, 0, 0, sprite.get_width(), sprite.get_height(), colorA);
}

pub fn draw_sprite_sized_source(sprite: &Sprite, posX: f32, posY: f32, width: f32, height: f32, srcPosX: u32, srcPosY: u32, srcWidth: u32, srcHeight: u32, colorA: u8)
{
    wein2dWASM_drawSprite(sprite.index, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA);
}

pub fn draw_text(content: &str, posX: f32, posY: f32, fontSize: f32, fontFamily: &str, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
{
    wein2dWASM_drawText(content, posX, posY, TextPositioning::RIGHT, fontSize, fontFamily, colorA, colorR, colorG, colorB);
}

pub fn draw_text_positioned(content: &str, posX: f32, posY: f32, positioning: &str, fontSize: f32, fontFamily: &str, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
{
    wein2dWASM_drawText(content, posX, posY, positioning, fontSize, fontFamily, colorA, colorR, colorG, colorB);
}

pub fn fill(colorA: u8, colorR: u8, colorG: u8, colorB: u8)
{
    wein2dWASM_fill(colorA, colorR, colorG, colorB);
}

pub fn draw_line(posX: f32, posY: f32, endX: f32, endY: f32, lineWidth: f32, colorA: u8, colorR: u8, colorG: u8, colorB: u8)
{
    wein2dWASM_drawLine(posX, posY, endX, endY, lineWidth, colorA, colorR, colorG, colorB);
}

pub fn draw_virtual_canvas(virtualCanvas: &VirtualCanvas, posX: f32, posY: f32, colorA: u8)
{
    wein2dWASM_drawVirtualCanvas(virtualCanvas.index, posX, posY, virtualCanvas.width as f32, virtualCanvas.height as f32, 0f32, 0f32, virtualCanvas.width as f32, virtualCanvas.height as f32, colorA);
}

pub fn draw_virtual_canvas_sized(virtualCanvas: &VirtualCanvas, posX: f32, posY: f32, width: f32, height: f32, colorA: u8)
{
    wein2dWASM_drawVirtualCanvas(virtualCanvas.index, posX, posY, width, height, 0f32, 0f32, virtualCanvas.width as f32, virtualCanvas.height as f32, colorA);
}

pub fn draw_virtual_canvas_sized_source(virtualCanvas: &VirtualCanvas, posX: f32, posY: f32, width: f32, height: f32, srcPosX: f32, srcPosY: f32, srcWidth: f32, srcHeight: f32, colorA: u8)
{
    wein2dWASM_drawVirtualCanvas(virtualCanvas.index, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA);
}
