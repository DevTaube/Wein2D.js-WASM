/*
 * Copyright (c) 2022, DevTaube
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     Redistributions of source code must retain the above copyright notice,
 *     this list of conditions and the following disclaimer.
 *
 *     Redistributions in binary form must reproduce the above copyright notice,
 *     this list of conditions and the following disclaimer in the documentation
 *     and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// Code for initialization, wein2dWASM_onStart and wein2dWASM_onFrame

let wein2dWASM_wasm

let wein2dWASM_initialized = false

function wein2dWASM_init(canvasElement, wasmpackLinkerFile)
{
    import(wasmpackLinkerFile).then( (wasm_linker) => {
        wasm_linker.default().then(() => {

            if(typeof wasm_linker.on_start !== "function")
            {
                throw `Provided wasm-linker-module does not have a function called "on_start".

Implementation in Rust (requires crate "wasm-bindgen"):
==================================================

// needed only once at start of file
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn on_start()
{
    // code to run on initialization here
}

==================================================`;
            }

            if(typeof wasm_linker.on_frame !== "function")
            {
                throw `Provided wasm-linker-module does not have a function called "on_frame".

Implementation in Rust (requires crate "wasm-bindgen"):
==================================================

// needed only once at start of file
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn on_frame()
{
    // code to run on every frame here
}

==================================================`;
            }

            wein2dWASM_wein2d = new Wein2DApplication(canvasElement, wein2dWASM_onFrame)
            wein2dWASM_wasm = wasm_linker
            wein2dWASM_onStart()
            console.info('%c[Wein2D.js-WASM] WebAssembly binary loaded successfully.', 'color: #bada55');
        })
    })
}

function wein2dWASM_onStart()
{
    wein2dWASM_wasm.on_start()
    wein2dWASM_initialized = true
}

function wein2dWASM_onFrame()
{
    if(!wein2dWASM_initialized)
        return

    wein2dWASM_wasm.on_frame()
}

// Wein2DApplication functionality

let wein2dWASM_wein2d

function wein2dWASM_getWidth()
{
    return wein2dWASM_wein2d.width
}

function wein2dWASM_getHeight()
{
    return wein2dWASM_wein2d.height
}

function wein2dWASM_getDeltaTime()
{
    return wein2dWASM_wein2d.deltaTime
}

function wein2dWASM_getKey(key)
{
    return wein2dWASM_wein2d.getKey(key)
}

function wein2dWASM_getTypedText()
{
    return wein2dWASM_wein2d.getTypedText()
}

function wein2dWASM_setTypedText(text)
{
    wein2dWASM_wein2d.setTypedText(text)
}

function wein2dWASM_getMouseX()
{
    return wein2dWASM_wein2d.getMouseX()
}

function wein2dWASM_getMouseY()
{
    return wein2dWASM_wein2d.getMouseY()
}

function wein2dWASM_getMouseL()
{
    return wein2dWASM_wein2d.getMouseL()
}

function wein2dWASM_getMouseS()
{
    return wein2dWASM_wein2d.getMouseS()
}

function wein2dWASM_getMouseR()
{
    return wein2dWASM_wein2d.getMouseR()
}

function wein2dWASM_drawRectangle()
{
    return wein2dWASM_rendercalls.add(new RectangleRenderCall(wein2dWASM_wein2d))
}

function wein2dWASM_drawOval()
{
    return wein2dWASM_rendercalls.add(new OvalRenderCall(wein2dWASM_wein2d))
}

function wein2dWASM_drawSprite()
{
    return wein2dWASM_rendercalls.add(new SpriteRenderCall(wein2dWASM_wein2d))
}

function wein2dWASM_drawVirtualCanvas()
{
    return wein2dWASM_rendercalls.add(new VirtualCanvasRenderCall(wein2dWASM_wein2d))
}

function wein2dWASM_drawText()
{
    return wein2dWASM_rendercalls.add(new TextRenderCall(wein2dWASM_wein2d))
}

function wein2dWASM_drawLine()
{
    return wein2dWASM_rendercalls.add(new LineRenderCall(wein2dWASM_wein2d))
}

function wein2dWASM_clearRectangle()
{
    return wein2dWASM_rendercalls.add(new ClearRectangleRenderCall(wein2dWASM_wein2d))
}

function wein2dWASM_fill(red, green, blue, alpha)
{
    wein2dWASM_wein2d.fill(red, green, blue, alpha)
}

// Code for collections (used for storing sounds, sprites, virtualcanvases and rendercalls)

class Collection
{

    items = []
    freeIndexes = []

    add(item)
    {
        if(this.freeIndexes.length > 0)
        {
            let itemIndex = this.freeIndexes.shift()
            this.items[itemIndex] = item
            return itemIndex
        }

        let itemIndex = this.items.length
        this.items[itemIndex] = item
        return itemIndex
    }

    get(index)
    {
        return this.items[index]
    }

    drop(index)
    {
        this.items[index] = undefined
        this.freeIndexes.push(index)
    }

}

// Sound Collection

let wein2dWASM_sounds = new Collection()

function wein2dWASM_sound_create(filePath)
{
    return wein2dWASM_sounds.add(new Sound(filePath))
}

function wein2dWASM_sound_setVolume(index, volume)
{
    wein2dWASM_sounds.get(index).setVolume(volume)
}

function wein2dWASM_sound_play(index)
{
    wein2dWASM_sounds.get(index).play()
}

function wein2dWASM_sound_loop(index, looping)
{
    wein2dWASM_sounds.get(index).loop(looping)
}

function wein2dWASM_sound_stop(index)
{
    wein2dWASM_sounds.get(index).stop()
}

function wein2dWASM_sound_isPlaying(index)
{
    return wein2dWASM_sounds.get(index).isPlaying()
}

function wein2dWASM_sound_drop(index)
{
    wein2dWASM_sounds.drop(index)
}

// sprite collection

let wein2dWASM_sprites = new Collection()

function wein2dWASM_sprite_create(filePath)
{
    return wein2dWASM_sprites.add(new Sprite(filePath))
}

function wein2dWASM_sprite_getWidth(index)
{
    return wein2dWASM_sprites.get(index).getWidth()
}

function wein2dWASM_sprite_getHeight(index)
{
    return wein2dWASM_sprites.get(index).getHeight()
}

function wein2dWASM_sprite_drop(index)
{
    wein2dWASM_sprites.drop(index)
}

// virtualCanvases collection

let wein2dWASM_virtualCanvases = new Collection()

function wein2dWASM_virtualcanvas_create(width, height)
{
    return wein2dWASM_virtualCanvases.add(new VirtualCanvas(width, height))
}

function wein2dWASM_virtualcanvas_drawRectangle(index)
{
    return wein2dWASM_rendercalls.add(new RectangleRenderCall(wein2dWASM_virtualCanvases.get(index)))
}

function wein2dWASM_virtualcanvas_drawOval(index)
{
    return wein2dWASM_rendercalls.add(new OvalRenderCall(wein2dWASM_virtualCanvases.get(index)))
}

function wein2dWASM_virtualcanvas_drawSprite(index)
{
    return wein2dWASM_rendercalls.add(new SpriteRenderCall(wein2dWASM_virtualCanvases.get(index)))
}

function wein2dWASM_virtualcanvas_drawVirtualCanvas(index)
{
    return wein2dWASM_rendercalls.add(new VirtualCanvasRenderCall(wein2dWASM_virtualCanvases.get(index)))
}

function wein2dWASM_virtualcanvas_drawText(index)
{
    return wein2dWASM_rendercalls.add(new TextRenderCall(wein2dWASM_virtualCanvases.get(index)))
}

function wein2dWASM_virtualcanvas_drawLine(index)
{
    return wein2dWASM_rendercalls.add(new LineRenderCall(wein2dWASM_virtualCanvases.get(index)))
}

function wein2dWASM_virtualcanvas_clearRectangle(index)
{
    return wein2dWASM_rendercalls.add(new ClearRectangleRenderCall(wein2dWASM_virtualCanvases.get(index)))
}

function wein2dWASM_virtualCanvas_fill(index, red, green, blue, alpha)
{
    wein2dWASM_virtualCanvases.get(index).fill(red, green, blue, alpha)
}

function wein2dWASM_virtualcanvas_getWidth(index)
{
    return wein2dWASM_virtualCanvases.get(index).getWidth()
}

function wein2dWASM_virtualcanvas_getHeight(index)
{
    return wein2dWASM_virtualCanvases.get(index).getHeight()
}

function wein2dWASM_virtualcanvas_drop(index)
{
    wein2dWASM_virtualCanvases.drop(index)
}

// rendercalls collections

let wein2dWASM_rendercalls = new Collection()

function wein2dWASM_rendercall_setPosition(index, x, y)
{
    wein2dWASM_rendercalls.get(index).setPosition(x, y)
}

function wein2dWASM_rendercall_setSize(index, width, height)
{
    wein2dWASM_rendercalls.get(index).setSize(width, height)
}

function wein2dWASM_rendercall_setColor(index, red, green, blue, alpha)
{
    wein2dWASM_rendercalls.get(index).setColor(red, green, blue, alpha)
}

function wein2dWASM_rendercall_rotateRadians(index, angle)
{
    wein2dWASM_rendercalls.get(index).rotateRadians(angle)
}

function wein2dWASM_rendercall_rotateRadiansPosition(index, angle, x, y)
{
    wein2dWASM_rendercalls.get(index).rotateRadians(angle, x, y)
}

function wein2dWASM_rendercall_draw(index)
{
    wein2dWASM_rendercalls.get(index).draw()
}

function wein2dWASM_rendercall_setSprite(index, spriteIndex)
{
    wein2dWASM_rendercalls.get(index).setSprite(wein2dWASM_sprites.get(spriteIndex))
}

function wein2dWASM_rendercall_setAlpha(index, alpha)
{
    wein2dWASM_rendercalls.get(index).setAlpha(alpha)
}

function wein2dWASM_rendercall_setSpriteCutoutDimensions(index, x, y, width, height)
{
    wein2dWASM_rendercalls.get(index).setSpriteCutoutDimensions(x, y, width, height)
}

function wein2dWASM_rendercall_setVirtualCanvas(index, canvasIndex)
{
    wein2dWASM_rendercalls.get(index).setVirtualCanvas(wein2dWASM_virtualCanvases.get(canvasIndex))
}

function wein2dWASM_rendercall_setCanvasCutoutDimensions(index, x, y, width, height)
{
    wein2dWASM_rendercalls.get(index).setCanvasCutoutDimensions(x, y, width, height)
}

function wein2dWASM_rendercall_setTextContent(index, content)
{
    wein2dWASM_rendercalls.get(index).setContent(content)
}

function wein2dWASM_rendercall_setTextSize(index, size)
{
    wein2dWASM_rendercalls.get(index).setSize(size)
}

function wein2dWASM_rendercall_setPositioning(index, textPositioning)
{
    wein2dWASM_rendercalls.get(index).setPositioning(textPositioning)
}

function wein2dWASM_rendercall_setFontFamily(index, fontFamily)
{
    wein2dWASM_rendercalls.get(index).setFontFamily(fontFamily)
}

function wein2dWASM_rendercall_setStart(index, x, y)
{
    wein2dWASM_rendercalls.get(index).setStart(x, y)
}

function wein2dWASM_rendercall_setEnd(index, x, y)
{
    wein2dWASM_rendercalls.get(index).setEnd(x, y)
}

function wein2dWASM_rendercall_setWidth(index, width)
{
    wein2dWASM_rendercalls.get(index).setWidth(width)
}

function wein2dWASM_rendercall_drop(index)
{
    wein2dWASM_rendercalls.drop(index)
}
