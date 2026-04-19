use alloc::boxed::Box;
use alloc::vec;

use bindings::future::{Executor, sleep};
use bindings::graphics::{self, *};
use bindings::include_res_lz;
use bindings::pokeemerald::*;
use bindings::resources::{AllocBuf, Buffer};

mod nmos;
mod slides;
mod ui;

use slides::*;
use ui::*;

include_res_lz!(TILESET, "../../graphics/party_menu_full/tiles.4bpp");
include_res_lz!(PAL, "../../graphics/party_menu_full/tiles.gbapal");
include_res_lz!(SCROLL_BG_MAP, "../../graphics/party_menu_full/bg.bin");

static EXECUTOR: Executor = Executor::new();

#[unsafe(no_mangle)]
extern "C" fn InitPresentation() {
    let fut = Box::new(presentation());

    unsafe { SetMainCallback2(Some(main_cb)) }
    EXECUTOR.set(fut);
}

extern "C" fn main_cb() {
    EXECUTOR.poll();
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}

extern "C" fn vblank_cb() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ChangeBgX(3, 64, BG_COORD_ADD as _);
        ChangeBgY(3, 64, BG_COORD_ADD as _);
    }
}

async fn slide_n(context: &Context<'_>, n: usize) -> bool {
    match n {
        0 => slide_intro(context).await,
        1 => slide_ipmx_what(context).await,
        2 => slide_st2110(context).await,
        3 => slide_nmos(context).await,
        4 => slide_nmos_live(context).await,
        5 => slide_vsf(context).await,
        6 => slide_ipmx_value(context).await,
        7 => slide_benefits(context).await,
        8 => slide_example(context).await,
        9 => slide_benefits_2(context).await,
        10 => slide_why_now(context).await,
        11 => slide_why_now_2(context).await,
        12 => slide_anim_generated(context).await,
        13 => slide_thank_you(context).await,
        _ => return false,
    }
    true
}

async fn presentation() {
    graphics::fade_palette(PaletteMask::ALL, 5, 0, 16, 0).await;
    clear_ui().await;
    set_gpu_registers(&[(REG_OFFSET_DISPCNT, &[DISPCNT_OBJ_ON, DISPCNT_OBJ_1D_MAP])]);

    let palettes: [BgPalette; 6] = load_bg_palettes(0, &PAL.load().get());
    let tileset: AllocBuf<TileBitmap4bpp> = TILESET.load();
    let tileset = Tileset {
        char_base: 1,
        offset: 0,
        tiles: &tileset,
        palette: palettes[0],
    };
    sleep(1).await;

    let bg_map: AllocBuf<Tile4bpp> = SCROLL_BG_MAP.load();
    let bg_size = bg_map.size_bytes();
    let bg_map = Tilemap {
        map: 0,
        buffer: &bg_map,
    };
    sleep(1).await;

    let bg = Background::load(BackgroundIndex::Background3, 3, tileset, bg_map).await;
    let bg = bg.handle();
    bg.set_pos(0, 0);
    bg.copy_tilemap_to_vram();
    bg.show();

    let map = Tilemap {
        map: 2,
        buffer: AllocBuf::new(vec![0u8; bg_size].into_boxed_slice()),
    };
    let fg = Background::load(BackgroundIndex::Background2, 2, tileset, map).await;
    let fg = fg.handle();
    fg.show();

    let msg_box = load_msg_box_gfx(fg, 0x20, 14);
    let border_gfx = load_user_window_gfx(fg, 0x50, 15);

    let context = Context {
        bg,
        fg,
        msg_box,
        border_gfx,
    };

    unsafe { SetVBlankCallback(Some(vblank_cb)) };
    let mut i = 0;
    while slide_n(&context, i).await {
        i += 1;
    }

    loop {
        sleep(1).await
    }
}
