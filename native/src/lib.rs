use neon::prelude::*;

use euclid::default::Point3D;
use physx_sys::*;
use std::ptr::null_mut;
pub mod blocks;
pub mod rendering;
pub mod utils;

pub fn log<'a, T: Context<'a>>(cx: &mut T, str: &str) {
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

pub fn example_chunk_mesh(mut cx: FunctionContext) -> JsResult<JsObject> {
    println!("Loading Chunks...");
    let mut mesh = rendering::Mesh::empty();
    let size = cx.argument::<JsNumber>(0)?.value() as i64;
    for x in -size..=size {
        for y in -size..=size {
            for z in -size..=size {
                println!("Loading Chunk at ({}, {}, {})", x, y, z);
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
    println!("Finished Loading Chunks");
    Ok(obj)
}

fn hello(mut cx: FunctionContext) -> JsResult<JsNull> {
    unsafe {
        let foundation = physx_create_foundation();
        let physics = physx_create_physics(foundation);

        let mut scene_desc = PxSceneDesc_new(PxPhysics_getTolerancesScale(physics));
        scene_desc.gravity = PxVec3 {
            x: 0.0,
            y: -9.81,
            z: 0.0,
        };

        let dispatcher = phys_PxDefaultCpuDispatcherCreate(1, null_mut());
        scene_desc.cpuDispatcher = dispatcher as *mut PxCpuDispatcher;
        scene_desc.filterShader = get_default_simulation_filter_shader();

        let scene = PxPhysics_createScene_mut(physics, &scene_desc);

        let material = PxPhysics_createMaterial_mut(physics, 0.5, 0.5, 0.6);
        let ground_plane =
            phys_PxCreatePlane(physics, &PxPlane_new_1(0.0, 1.0, 0.0, 0.0), material);
        PxScene_addActor_mut(scene, ground_plane as *mut PxActor, null_mut());

        let sphere_geo = PxSphereGeometry_new_1(10.0);
        let sphere = phys_PxCreateDynamic(
            physics,
            &PxTransform_new_1(&PxVec3 {
                x: 0.0,
                y: 40.0,
                z: 100.0,
            }),
            &sphere_geo as *const PxSphereGeometry as *const PxGeometry,
            material,
            10.0,
            &PxTransform_new_2(PxIdentity),
        );
        PxRigidBody_setAngularDamping_mut(sphere as *mut PxRigidBody, 0.5);
        PxScene_addActor_mut(scene, sphere as *mut PxActor, null_mut());

        let heights_over_time = (0..100)
            .map(|_| {
                PxScene_simulate_mut(scene, 0.1, null_mut(), null_mut(), 0, true);
                let mut error: u32 = 0;
                PxScene_fetchResults_mut(scene, true, &mut error);
                assert!(error == 0, "fetchResults has failed");
                let pose = PxRigidActor_getGlobalPose(sphere as *mut PxRigidActor);
                (pose.p.y) as i32 - 10
            })
            .collect::<Vec<_>>();
        let max_h = 18;
        let s = (0..max_h)
            .map(|h| {
                let h = max_h - 1 - h;
                heights_over_time
                    .iter()
                    .enumerate()
                    .map(|(_t, p)| if h == *p { 'o' } else { ' ' })
                    .collect::<String>()
            })
            .map(|str| str + "\n")
            .collect::<String>();
        log(&mut cx, &s[..]);
        PxScene_release_mut(scene);
        PxDefaultCpuDispatcher_release_mut(dispatcher);
        PxPhysics_release_mut(physics);
        Ok(cx.null())
    }
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("example_chunk_mesh", example_chunk_mesh)
});

#[cfg(test)]
mod tests;
