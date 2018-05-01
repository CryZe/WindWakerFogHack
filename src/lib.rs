#![no_std]
#![feature(lang_items, core_float)]
pub mod lang_items;

// pub mod libc;

#[macro_use]
extern crate libtww;

use libtww::system::{game_info, gx, j3d};

mod voxels;

use voxels::{blend, marching_cubes, sphere, ArrayView3D, Vec3};

struct State {
    // tex_obj: gx::TexObj,
    voxels: ArrayView3D,
    // world: World<f32>,
    // balls: Vec<RigidBodyHandle<f32>>,
}

static mut STATE: Option<State> = None;

unsafe fn get_state() -> &'static mut State {
    // #[repr(align(32))]
    // struct TextureData([u8; 41472]);
    // static TEXTURE_DATA: TextureData = TextureData(*include_bytes!("texture.bin"));

    STATE.get_or_insert_with(|| {
        // let mut tex_obj = gx::TexObj::default();
        // gx::init_tex_obj(
        //     &mut tex_obj,
        //     TEXTURE_DATA.0.as_ptr() as *const u8,
        //     144,
        //     144,
        //     gx::TF_RGB5A3,
        //     gx::CLAMP,
        //     gx::CLAMP,
        //     gx::FALSE,
        // );

        State {
            // tex_obj,
            voxels: ArrayView3D::new((20, 10, 20)),
            // world: create_world(),
            // balls: Vec::new(),
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn game_loop() {
    let state = get_state();

    use core::num::Float;
    for cell in &mut state.voxels.data {
        *cell = (*cell + 0.0002).min(1.0);
    }

    let pos = libtww::Link::position();
    sphere::sphere(
        &mut state.voxels,
        Vec3::new(
            (pos.x + 202_000.0) / 50.0,
            (pos.y + 100.0) / 50.0,
            (pos.z - 315_500.0) / 50.0,
        ),
        1.5,
        1.0,
        0.1,
        blend::Operation::Sub,
    );
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    let state = get_state();

    gx::set_z_mode(gx::ENABLE, gx::LEQUAL, gx::TRUE);

    let projection_base = game_info::GAME_INFO.projection_base;
    if !projection_base.is_null() {
        gx::set_projection(&mut (*projection_base).projection_mtx, gx::PERSPECTIVE);
    }

    // gx::clear_vtx_desc();
    // gx::set_vtx_desc(gx::VA_POS as u8, gx::DIRECT);
    // gx::set_vtx_desc(gx::VA_CLR0 as u8, gx::DIRECT);
    // gx::set_vtx_desc(gx::VA_TEX0 as u8, gx::DIRECT);

    // gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_TEX0, gx::TEX_ST, gx::F32, 0);
    // gx::set_num_tex_gens(1);
    // gx::set_tex_coord_gen(
    //     gx::TEXCOORD0 as u16,
    //     gx::TG_MTX2x4,
    //     gx::TG_TEX0,
    //     gx::IDENTITY,
    // );

    // gx::set_tev_op(gx::TEVSTAGE0, gx::REPLACE);
    // gx::set_tev_order(gx::TEVSTAGE0, gx::TEXCOORD0, gx::TEXMAP0, gx::COLOR0A0);
    // gx::load_tex_obj(&mut state.tex_obj, gx::TEXMAP0 as u8);

    // let (top, bottom) = (0xFF_00_00_FF, 0x00_FF_FF_FF);
    // let (top, bottom) = (0xFF_00_00_FF, 0x00_FF_FF_FF);

    gx::set_cull_mode(gx::CULL_BACK);

    gx::set_blend_mode(
        gx::BM_BLEND,
        gx::BL_SRCALPHA,
        gx::BL_INVSRCALPHA,
        gx::LO_SET,
    );
    gx::load_pos_mtx_imm(&mut j3d::CAMERA_MATRIX, gx::PNMTX0);
    gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_POS, gx::POS_XYZ, gx::F32, 0);

    marching_cubes::march(&state.voxels, 0.5, |t| {
        gx::begin(gx::TRIANGLES, gx::VTXFMT0, 3);
        {
            for &vertex in &[&t.c, &t.b, &t.a] {
                let coord = &vertex.coord;
                gx::submit_f32s(&[
                    50.0 * coord.x - 202_000.0,
                    50.0 * coord.y - 100.0,
                    50.0 * coord.z + 315_500.0,
                ]);
                gx::submit_u32(0xD0_D0_D0_F0);
            }
        }
        gx::end();
    });

    // gx::begin(gx::QUADS, gx::VTXFMT0, 4);
    // {
    //     gx::submit_f32s(&[-205_000.0, 0.0, 315_000.0]);
    //     gx::submit_u32(top);
    //     // gx::submit_f32s(&[0.0, 1.0]);

    //     gx::submit_f32s(&[-200_000.0, 0.0, 315_000.0]);
    //     gx::submit_u32(top);
    //     // gx::submit_f32s(&[1.0, 1.0]);

    //     gx::submit_f32s(&[-200_000.0, 500_0.0, 315_000.0]);
    //     gx::submit_u32(bottom);
    //     // gx::submit_f32s(&[1.0, 0.0]);

    //     gx::submit_f32s(&[-205_000.0, 500_0.0, 315_000.0]);
    //     gx::submit_u32(bottom);
    //     // gx::submit_f32s(&[0.0, 0.0]);
    // }
    // gx::end();
    gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_POS, gx::POS_XYZ, gx::S16, 0);
}
