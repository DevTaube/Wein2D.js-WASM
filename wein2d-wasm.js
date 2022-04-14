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
                console.error(`Provided wasm-linker-module does not have a function called "on_start".

Implementation in Rust (requires crate "wasm-bindgen"):
==================================================

// needed only once at start of file
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn on_start()
{
    // code to run on initialization here
}

==================================================`)
                return
            }

            if(typeof wasm_linker.on_frame !== "function")
            {
                console.error(`Provided wasm-linker-module does not have a function called "on_frame".

Implementation in Rust (requires crate "wasm-bindgen"):
==================================================

// needed only once at start of file
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn on_frame()
{
    // code to run on every frame here
}

==================================================`)
                return
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

// Sound

let wein2dWASM_sounds = []
let wein2dWASM_freeSoundIndexes = []

function wein2dWASM_sound_new(filePath)
{
    let newSound = new Sound(filePath)

    if(wein2dWASM_freeSoundIndexes.length > 0)
    {
        let newSoundIndex = wein2dWASM_freeSoundIndexes.shift()
        wein2dWASM_sounds[newSoundIndex] = newSound
        return newSoundIndex
    }

    let newSoundIndex = wein2dWASM_sounds.length
    wein2dWASM_sounds[newSoundIndex] = newSound
    return newSoundIndex
}

function wein2dWASM_sound_setVolume(index, volume)
{
    wein2dWASM_sounds[index].setVolume(volume)
}

function wein2dWASM_sound_play(index)
{
    wein2dWASM_sounds[index].play()
}

function wein2dWASM_sound_loop(index, looping)
{
    wein2dWASM_sounds[index].loop(looping)
}

function wein2dWASM_sound_stop(index)
{
    wein2dWASM_sounds[index].stop()
}

function wein2dWASM_sound_isPlaying(index)
{
    return wein2dWASM_sounds[index].isPlaying()
}

function wein2dWASM_sound_drop(index)
{
    wein2dWASM_sounds[index] = undefined
    wein2dWASM_freeSoundIndexes.push(index)
}

// Sprite

let wein2dWASM_sprites = []
let wein2dWASM_freeSpriteIndexes = []

function wein2dWASM_sprite_new(filePath)
{
    let newSprite = new Sprite(filePath)

    if(wein2dWASM_freeSpriteIndexes.length > 0)
    {
        let newSpriteIndex = wein2dWASM_freeSpriteIndexes.shift()
        wein2dWASM_sprites[newSpriteIndex] = newSprite
        return newSpriteIndex
    }

    let newSpriteIndex = wein2dWASM_sprites.length
    wein2dWASM_sprites[newSpriteIndex] = newSprite
    return newSpriteIndex
}

function wein2dWASM_sprite_getWidth(index)
{
    return wein2dWASM_sprites[index].getWidth()
}

function wein2dWASM_sprite_getHeight(index)
{
    return wein2dWASM_sprites[index].getHeight()
}

function wein2dWASM_sprite_drop(index)
{
    wein2dWASM_sprites[index] = undefined
    wein2dWASM_freeSpriteIndexes.push(index)
}

// VirtualCanvas

let wein2dWASM_virtualCanvases = []
let wein2dWASM_freeVirtualCanvasIndexes = []

function wein2dWASM_virtualCanvas_new(width, height)
{
    let newVirtualCanvas = new VirtualCanvas(width, height)

    if(wein2dWASM_freeVirtualCanvasIndexes.length > 0)
    {
        let newVirtualCanvasIndex = wein2dWASM_freeVirtualCanvasIndexes.shift()
        wein2dWASM_virtualCanvases[newVirtualCanvasIndex] = newVirtualCanvas
        return newVirtualCanvasIndex
    }

    let newVirtualCanvasIndex = wein2dWASM_virtualCanvases.length
    wein2dWASM_virtualCanvases[newVirtualCanvasIndex] = newVirtualCanvas
    return newVirtualCanvasIndex
}

function wein2dWASM_virtualCanvas_getWidth(index)
{
    return wein2dWASM_virtualCanvases[index].getWidth()
}

function wein2dWASM_virtualCanvas_getHeight(index)
{
    return wein2dWASM_virtualCanvases[index].getHeight()
}

function wein2dWASM_virtualCanvas_drawRect(destVirtualCanvasIndex, posX, posY, width, height, colorA, colorR, colorG, colorB)
{
    wein2dWASM_virtualCanvases[destVirtualCanvasIndex].drawRect(posX, posY, width, height, colorA, colorR, colorG, colorB)
}

function wein2dWASM_virtualCanvas_drawOval(destVirtualCanvasIndex, posX, posY, width, height, colorA, colorR, colorG, colorB)
{
    wein2dWASM_virtualCanvases[destVirtualCanvasIndex].drawOval(posX, posY, width, height, colorA, colorR, colorG, colorB)
}

function wein2dWASM_virtualCanvas_drawSprite(destVirtualCanvasIndex, spriteIndex, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
{
    wein2dWASM_virtualCanvases[destVirtualCanvasIndex].drawSprite(wein2dWASM_sprites[spriteIndex], posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
}

function wein2dWASM_virtualCanvas_drawText(destVirtualCanvasIndex, content, posX, posY, positioning, fontSize, fontFamily, colorA, colorR, colorG, colorB)
{
    wein2dWASM_virtualCanvases[destVirtualCanvasIndex].drawText(content, posX, posY, positioning, fontSize, fontFamily, colorA, colorR, colorG, colorB)
}

function wein2dWASM_virtualCanvas_fill(destVirtualCanvasIndex, colorA, colorR, colorG, colorB)
{
    wein2dWASM_virtualCanvases[destVirtualCanvasIndex].fill(colorA, colorR, colorG, colorB)
}

function wein2dWASM_virtualCanvas_drawLine(destVirtualCanvasIndex, posX, posY, endX, endY, lineWidth, colorA, colorR, colorG, colorB)
{
    wein2dWASM_virtualCanvases[destVirtualCanvasIndex].drawLine(posX, posY, endX, endY, lineWidth, colorA, colorR, colorG, colorB)
}

function wein2dWASM_virtualCanvas_drawVirtualCanvas(destVirtualCanvasIndex, virtualCanvasIndex, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
{
    wein2dWASM_virtualCanvases[destVirtualCanvasIndex].drawVirtualCanvas(wein2dWASM_virtualCanvases[virtualCanvasIndex], posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
}

function wein2dWASM_virtualCanvas_drop(index)
{
    wein2dWASM_virtualCanvases[index] = undefined
    wein2dWASM_freeVirtualCanvasIndexes.push(index)
}

// Wein2DApplication

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

function wein2dWASM_drawRect(posX, posY, width, height, colorA, colorR, colorG, colorB)
{
    wein2dWASM_wein2d.drawRect(posX, posY, width, height, colorA, colorR, colorG, colorB)
}

function wein2dWASM_drawOval(posX, posY, width, height, colorA, colorR, colorG, colorB)
{
    wein2dWASM_wein2d.drawOval(posX, posY, width, height, colorA, colorR, colorG, colorB)
}

function wein2dWASM_drawSprite(spriteIndex, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
{
    wein2dWASM_wein2d.drawSprite(wein2dWASM_sprites[spriteIndex], posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
}

function wein2dWASM_drawText(content, posX, posY, positioning, fontSize, fontFamily, colorA, colorR, colorG, colorB)
{
    wein2dWASM_wein2d.drawText(content, posX, posY, positioning, fontSize, fontFamily, colorA, colorR, colorG, colorB)
}

function wein2dWASM_fill(colorA, colorR, colorG, colorB)
{
    wein2dWASM_wein2d.fill(colorA, colorR, colorG, colorB)
}

function wein2dWASM_drawLine(posX, posY, endX, endY, lineWidth, colorA, colorR, colorG, colorB)
{
    wein2dWASM_wein2d.drawLine(posX, posY, endX, endY, lineWidth, colorA, colorR, colorG, colorB)
}

function wein2dWASM_drawVirtualCanvas(virtualCanvasIndex, posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
{
    wein2dWASM_wein2d.drawVirtualCanvas(wein2dWASM_virtualCanvases[virtualCanvasIndex], posX, posY, width, height, srcPosX, srcPosY, srcWidth, srcHeight, colorA)
}
