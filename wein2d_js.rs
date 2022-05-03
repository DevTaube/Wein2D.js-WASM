#![allow(non_snake_case)]
#![allow(dead_code)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern
{
    fn wein2dWASM_sound_create(file_path: &str) -> u32;
    fn wein2dWASM_sound_setVolume(index: u32, volume: f32);
    fn wein2dWASM_sound_play(index: u32);
    fn wein2dWASM_sound_loop(index: u32, looping: bool);
    fn wein2dWASM_sound_stop(index: u32);
    fn wein2dWASM_sound_isPlaying(index: u32) -> bool;
    fn wein2dWASM_sound_drop(index: u32);

    fn wein2dWASM_sprite_create(file_path: &str) -> u32;
    fn wein2dWASM_sprite_getWidth(index: u32) -> u32;
    fn wein2dWASM_sprite_getHeight(index: u32) -> u32;
    fn wein2dWASM_sprite_drop(index: u32);

    fn wein2dWASM_virtualcanvas_create(width: u32, height: u32) -> u32;
    fn wein2dWASM_virtualcanvas_drawRectangle(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_drawOval(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_drawSprite(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_drawVirtualCanvas(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_drawText(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_drawLine(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_clearRectangle(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_fill(index: u32, red: u8, green: u8, blue: u8, alpha: u8);
    fn wein2dWASM_virtualcanvas_getWidth(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_getHeight(index: u32) -> u32;
    fn wein2dWASM_virtualcanvas_drop(index: u32);

    fn wein2dWASM_rendercall_setPosition(index: u32, x: f32, y: f32);
    fn wein2dWASM_rendercall_setSize(index: u32, width: f32, height: f32);
    fn wein2dWASM_rendercall_setColor(index: u32, red: u8, green: u8, blue: u8, alpha: u8);
    fn wein2dWASM_rendercall_rotateRadians(index: u32, angle: f32);
    fn wein2dWASM_rendercall_rotateRadiansPosition(index: u32, angle: f32, x: f32, y: f32);
    fn wein2dWASM_rendercall_draw(index: u32);
    fn wein2dWASM_rendercall_setSprite(index: u32, spriteIndex: u32);
    fn wein2dWASM_rendercall_setAlpha(index: u32, alpha: u8);
    fn wein2dWASM_rendercall_setSpriteCutoutDimensions(index: u32, x: u32, y: u32, width: u32, height: u32);
    fn wein2dWASM_rendercall_setVirtualCanvas(index: u32, canvasIndex: u32);
    fn wein2dWASM_rendercall_setCanvasCutoutDimensions(index: u32, x: u32, y: u32, width: u32, height: u32);
    fn wein2dWASM_rendercall_setTextContent(index: u32, content: &str);
    fn wein2dWASM_rendercall_setTextSize(index: u32, size: f32);
    fn wein2dWASM_rendercall_setPositioning(index: u32, textPositioning: &str);
    fn wein2dWASM_rendercall_setFontFamily(index: u32, fontFamily: &str);
    fn wein2dWASM_rendercall_setStart(index: u32, x: f32, y: f32);
    fn wein2dWASM_rendercall_setEnd(index: u32, x: f32, y: f32);
    fn wein2dWASM_rendercall_setWidth(index: u32, width: f32);
    fn wein2dWASM_rendercall_drop(index: u32);

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
    fn wein2dWASM_drawRectangle() -> u32;
    fn wein2dWASM_drawOval() -> u32;
    fn wein2dWASM_drawSprite() -> u32;
    fn wein2dWASM_drawVirtualCanvas() -> u32;
    fn wein2dWASM_drawText() -> u32;
    fn wein2dWASM_drawLine() -> u32;
    fn wein2dWASM_clearRectangle() -> u32;
    fn wein2dWASM_fill(red: u8, green: u8, blue: u8, alpha: u8);
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

pub struct BoxCollision {}

impl BoxCollision
{
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
            index: wein2dWASM_sound_create(filePath)
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
        let spriteIndex = wein2dWASM_sprite_create(filePath);
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

// Various Rendercalls

pub struct RectangleRenderCall { index: u32 }

impl RectangleRenderCall
{
    pub fn set_position(self, x: f32, y: f32) -> RectangleRenderCall { wein2dWASM_rendercall_setPosition(self.index, x, y); self }
    pub fn set_size(self, width: f32, height: f32) -> RectangleRenderCall { wein2dWASM_rendercall_setSize(self.index, width, height); self }
    pub fn set_color(self, red: u8, green: u8, blue: u8, alpha: u8) -> RectangleRenderCall { wein2dWASM_rendercall_setColor(self.index, red, green, blue, alpha); self }
    pub fn rotate_degrees(self, angle: f32) -> RectangleRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle.to_radians()); self }
    pub fn rotate_positioned_degrees(self, angle: f32, x: f32, y: f32) -> RectangleRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle.to_radians(), x, y); self }
    pub fn rotate_radians(self, angle: f32) -> RectangleRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle); self }
    pub fn rotate_positioned_radians(self, angle: f32, x: f32, y: f32) -> RectangleRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle, x, y); self }
    pub fn draw(self) { wein2dWASM_rendercall_draw(self.index); }
}

impl Drop for RectangleRenderCall { fn drop(&mut self)
    {
        wein2dWASM_rendercall_drop(self.index);
    } }


pub struct OvalRenderCall { index: u32 }

impl OvalRenderCall
{
    pub fn set_position(self, x: f32, y: f32) -> OvalRenderCall { wein2dWASM_rendercall_setPosition(self.index, x, y); self }
    pub fn set_size(self, width: f32, height: f32) -> OvalRenderCall { wein2dWASM_rendercall_setSize(self.index, width, height); self }
    pub fn set_color(self, red: u8, green: u8, blue: u8, alpha: u8) -> OvalRenderCall { wein2dWASM_rendercall_setColor(self.index, red, green, blue, alpha); self }
    pub fn draw(self) { wein2dWASM_rendercall_draw(self.index); }
}

impl Drop for OvalRenderCall { fn drop(&mut self)
    {
        wein2dWASM_rendercall_drop(self.index);
    } }


pub struct SpriteRenderCall { index: u32 }

impl SpriteRenderCall
{
    pub fn set_sprite(self, sprite: &Sprite) -> SpriteRenderCall { wein2dWASM_rendercall_setSprite(self.index, sprite.index); self }
    pub fn set_position(self, x: f32, y: f32) -> SpriteRenderCall { wein2dWASM_rendercall_setPosition(self.index, x, y); self }
    pub fn set_size(self, width: f32, height: f32) -> SpriteRenderCall { wein2dWASM_rendercall_setSize(self.index, width, height); self }
    pub fn set_alpha(self, alpha: u8) -> SpriteRenderCall { wein2dWASM_rendercall_setAlpha(self.index, alpha); self }
    pub fn set_sprite_cutout_dimensions(self, x: u32, y: u32, width: u32, height: u32) -> SpriteRenderCall { wein2dWASM_rendercall_setSpriteCutoutDimensions(self.index, x, y, width, height); self }
    pub fn rotate_degrees(self, angle: f32) -> SpriteRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle.to_radians()); self }
    pub fn rotate_positioned_degrees(self, angle: f32, x: f32, y: f32) -> SpriteRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle.to_radians(), x, y); self }
    pub fn rotate_radians(self, angle: f32) -> SpriteRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle); self }
    pub fn rotate_positioned_radians(self, angle: f32, x: f32, y: f32) -> SpriteRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle, x, y); self }
    pub fn draw(self) { wein2dWASM_rendercall_draw(self.index); }
}

impl Drop for SpriteRenderCall { fn drop(&mut self)
    {
        wein2dWASM_rendercall_drop(self.index);
    } }


pub struct VirtualCanvasRenderCall { index: u32 }

impl VirtualCanvasRenderCall
{
    pub fn set_virtual_canvas(self, virtual_canvas: &VirtualCanvas) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_setVirtualCanvas(self.index, virtual_canvas.index); self }
    pub fn set_position(self, x: f32, y: f32) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_setPosition(self.index, x, y); self }
    pub fn set_size(self, width: f32, height: f32) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_setSize(self.index, width, height); self }
    pub fn set_alpha(self, alpha: u8) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_setAlpha(self.index, alpha); self }
    pub fn set_canvas_cutout_dimensions(self, x: u32, y: u32, width: u32, height: u32) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_setCanvasCutoutDimensions(self.index, x, y, width, height); self }
    pub fn rotate_degrees(self, angle: f32) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle.to_radians()); self }
    pub fn rotate_positioned_degrees(self, angle: f32, x: f32, y: f32) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle.to_radians(), x, y); self }
    pub fn rotate_radians(self, angle: f32) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle); self }
    pub fn rotate_positioned_radians(self, angle: f32, x: f32, y: f32) -> VirtualCanvasRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle, x, y); self }
    pub fn draw(self) { wein2dWASM_rendercall_draw(self.index); }
}

impl Drop for VirtualCanvasRenderCall { fn drop(&mut self)
    {
        wein2dWASM_rendercall_drop(self.index);
    } }


pub struct TextRenderCall { index: u32 }

impl TextRenderCall
{
    pub fn set_content(self, content: &str) -> TextRenderCall { wein2dWASM_rendercall_setTextContent(self.index, content); self }
    pub fn set_position(self, x: f32, y: f32) -> TextRenderCall { wein2dWASM_rendercall_setPosition(self.index, x, y); self }
    pub fn set_size(self, size: f32) -> TextRenderCall { wein2dWASM_rendercall_setTextSize(self.index, size); self }
    pub fn set_positioning(self, positioning: &str) -> TextRenderCall { wein2dWASM_rendercall_setPositioning(self.index, positioning); self }
    pub fn set_font_family(self, family: &str) -> TextRenderCall { wein2dWASM_rendercall_setFontFamily(self.index, family); self }
    pub fn set_color(self, red: u8, green: u8, blue: u8, alpha: u8) -> TextRenderCall { wein2dWASM_rendercall_setColor(self.index, red, green, blue, alpha); self }
    pub fn draw(self) { wein2dWASM_rendercall_draw(self.index); }
}

impl Drop for TextRenderCall { fn drop(&mut self)
    {
        wein2dWASM_rendercall_drop(self.index);
    } }


pub struct LineRenderCall { index: u32 }

impl LineRenderCall
{
    pub fn set_start(self, x: f32, y: f32) -> LineRenderCall { wein2dWASM_rendercall_setStart(self.index, x, y); self }
    pub fn set_end(self, x: f32, y: f32) -> LineRenderCall { wein2dWASM_rendercall_setEnd(self.index, x, y); self }
    pub fn set_width(self, width: f32) -> LineRenderCall { wein2dWASM_rendercall_setWidth(self.index, width); self }
    pub fn set_color(self, red: u8, green: u8, blue: u8, alpha: u8) -> LineRenderCall { wein2dWASM_rendercall_setColor(self.index, red, green, blue, alpha); self }
    pub fn draw(self) { wein2dWASM_rendercall_draw(self.index); }
}

impl Drop for LineRenderCall { fn drop(&mut self)
    {
        wein2dWASM_rendercall_drop(self.index);
    } }


pub struct ClearRectangleRenderCall { index: u32 }

impl ClearRectangleRenderCall
{
    pub fn set_position(self, x: f32, y: f32) -> ClearRectangleRenderCall { wein2dWASM_rendercall_setPosition(self.index, x, y); self }
    pub fn set_size(self, width: f32, height: f32) -> ClearRectangleRenderCall { wein2dWASM_rendercall_setSize(self.index, width, height); self }
    pub fn rotate_degrees(self, angle: f32) -> ClearRectangleRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle.to_radians()); self }
    pub fn rotate_positioned_degrees(self, angle: f32, x: f32, y: f32) -> ClearRectangleRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle.to_radians(), x, y); self }
    pub fn rotate_radians(self, angle: f32) -> ClearRectangleRenderCall { wein2dWASM_rendercall_rotateRadians(self.index, angle); self }
    pub fn rotate_positioned_radians(self, angle: f32, x: f32, y: f32) -> ClearRectangleRenderCall { wein2dWASM_rendercall_rotateRadiansPosition(self.index, angle, x, y); self }
    pub fn draw(self) { wein2dWASM_rendercall_draw(self.index); }
}

impl Drop for ClearRectangleRenderCall { fn drop(&mut self)
    {
        wein2dWASM_rendercall_drop(self.index);
    } }

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
        VirtualCanvas
        {
            index: wein2dWASM_virtualcanvas_create(width, height),
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

    pub fn draw_rectangle(&self) -> RectangleRenderCall { RectangleRenderCall { index: wein2dWASM_virtualcanvas_drawRectangle(self.index) } }
    pub fn draw_oval(&self) -> OvalRenderCall { OvalRenderCall { index: wein2dWASM_virtualcanvas_drawOval(self.index) } }
    pub fn draw_sprite(&self) -> SpriteRenderCall { SpriteRenderCall { index: wein2dWASM_virtualcanvas_drawSprite(self.index) } }
    pub fn draw_virtual_canvas(&self) -> VirtualCanvasRenderCall { VirtualCanvasRenderCall { index: wein2dWASM_virtualcanvas_drawVirtualCanvas(self.index) } }
    pub fn draw_text(&self) -> TextRenderCall { TextRenderCall { index: wein2dWASM_virtualcanvas_drawText(self.index) } }
    pub fn draw_line(&self) -> LineRenderCall { LineRenderCall { index: wein2dWASM_virtualcanvas_drawLine(self.index) } }
    pub fn clear_rectangle(&self) -> ClearRectangleRenderCall { ClearRectangleRenderCall { index: wein2dWASM_virtualcanvas_clearRectangle(self.index) } }
}

impl Drop for VirtualCanvas
{
    fn drop(&mut self)
    {
        wein2dWASM_virtualcanvas_drop(self.index);
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

pub fn draw_rectangle() -> RectangleRenderCall { RectangleRenderCall { index: wein2dWASM_drawRectangle() } }
pub fn draw_oval() -> OvalRenderCall { OvalRenderCall { index: wein2dWASM_drawOval() } }
pub fn draw_sprite() -> SpriteRenderCall { SpriteRenderCall { index: wein2dWASM_drawSprite() } }
pub fn draw_virtual_canvas() -> VirtualCanvasRenderCall { VirtualCanvasRenderCall { index: wein2dWASM_drawVirtualCanvas() } }
pub fn draw_text() -> TextRenderCall { TextRenderCall { index: wein2dWASM_drawText() } }
pub fn draw_line() -> LineRenderCall { LineRenderCall { index: wein2dWASM_drawLine() } }
pub fn clear_rectangle() -> ClearRectangleRenderCall { ClearRectangleRenderCall { index: wein2dWASM_clearRectangle() } }
pub fn fill(red: u8, green: u8, blue: u8, alpha: u8) { wein2dWASM_fill(red, green, blue, alpha); }
