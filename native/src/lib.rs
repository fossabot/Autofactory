#![feature(arbitrary_self_types)]
//! The native component of the game.
//!
//! This will consist of everything apart from the user input and UI.

use neon::prelude::*;

use euclid::default::Point3D;
/// All block related stuff, including storage of blocks.
pub mod blocks;
/// All entity related stuff.
pub mod entity;
/// All rendering related stuff. This currently only includes a small mesh implementation that gets sent to the JavaScript part.
pub mod rendering;
/// Random stuff that doesn't belong anywhere else.
pub mod utils;

fn log<'a, T: Context<'a>>(cx: &mut T, str: &str) {
    let global = cx.global().downcast::<JsObject>().or_throw(cx).unwrap();
    let handle_console = global.get(cx, "console").unwrap();
    let console = handle_console.downcast::<JsObject>().or_throw(cx).unwrap();
    let log = console
        .get(cx, "log")
        .unwrap()
        .downcast::<JsFunction>()
        .unwrap();
    let s = cx.string(str);
    log.call(cx, handle_console, vec![s]).unwrap();
}

fn example_chunk_mesh(mut cx: FunctionContext) -> JsResult<JsObject> {
    log(&mut cx, "Loading Chunks...");
    let mut mesh = rendering::Mesh::empty();
    let size = cx.argument::<JsNumber>(0)?.value() as i64;
    for x in -size..=size {
        for y in -size..=size {
            for z in -size..=size {
                log(
                    &mut cx,
                    &format!("Loading Chunk at ({}, {}, {})", x, y, z)[..],
                );
                utils::generate_random_mesh(Point3D::new(x, y, z), &mut mesh);
            }
        }
    }
    let obj = JsObject::new(&mut cx);
    let index = utils::to_buffer(&mut cx, mesh.index)?;
    obj.set(&mut cx, "index", index)?;
    let positions = utils::to_buffer(&mut cx, mesh.positions)?;
    obj.set(&mut cx, "positions", positions)?;
    let normals = utils::to_buffer(&mut cx, mesh.normals)?;
    obj.set(&mut cx, "normals", normals)?;
    log(&mut cx, "Finished Loading Chunks");
    Ok(obj)
}

register_module!(mut cx, {
    cx.export_function("example_chunk_mesh", example_chunk_mesh)
});


pub fn main() {

}

#[cfg(test)]
mod tests;
