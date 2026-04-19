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

    // ── Act 1: intro ──────────────────────────────────────────────────────────
    slide_intro(&context).await;
    slide_ipmx_what(&context).await;
    slide_ipmx_value(&context).await;

    // ── Act 2: interactive hub — choose which component to explore ────────────
    let mut hub = HubState::new();
    loop {
        match slide_hub(&context, &hub).await {
            HubChoice::ST2110 => {
                slide_st2110(&context).await;
                hub.st2110_done = true;
            }
            HubChoice::NMOS => {
                slide_nmos(&context).await;
                slide_nmos_live(&context).await;
                hub.nmos_done = true;
            }
            HubChoice::VSF => {
                slide_vsf(&context).await;
                hub.vsf_done = true;
            }
            HubChoice::Continue => break,
        }
    }

    // ── Act 3: overview and conclusion ────────────────────────────────────────
    slide_benefits(&context).await;
    slide_example(&context).await;
    slide_benefits_2(&context).await;
    slide_why_now(&context).await;
    slide_why_now_2(&context).await;
    slide_anim_generated(&context).await;
    slide_thank_you(&context).await;

    loop {
        sleep(1).await
    }
}
