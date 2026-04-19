use alloc::boxed::Box;

use bindings::future::sleep;
use bindings::graphics::{self, SpriteSheet, Vec2D, Window, *};
use bindings::input::Button;
use bindings::pokeemerald::*;
use bindings::resources::{AllocBuf, Buffer};
use bindings::{include_res_lz, pkstr};

use super::nmos::*;
use super::ui::*;

// ─── Hub slide ───────────────────────────────────────────────────────────────

pub struct HubState {
    pub st2110_done: bool,
    pub nmos_done: bool,
    pub vsf_done: bool,
}

impl HubState {
    pub fn new() -> Self {
        Self { st2110_done: false, nmos_done: false, vsf_done: false }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum HubChoice {
    ST2110,
    NMOS,
    VSF,
    Continue,
}

fn redraw_hub_menu(window: &Window, context: &Context<'_>, state: &HubState, cursor: usize) {
    window.fill(1);
    window.draw_border(context.border_gfx);
    window.put_tilemap();

    // Three topics side by side in thirds of the 240px window
    macro_rules! topic {
        ($i:expr, $x:expr, $label:literal, $done:expr) => {
            print_text(window, font(), ($x, 4u8), if cursor == $i { pkstr!(b">") } else { pkstr!(b" ") });
            print_text(window, if cursor == $i { blue_font() } else { font() }, ($x + 8u8, 4u8), pkstr!($label));
            if $done {
                print_text(window, red_font(), ($x + 12u8, 16u8), pkstr!(b"[done]"));
            }
        };
    }

    topic!(0,   4u8, b"ST 2110",   state.st2110_done);
    topic!(1,  87u8, b"VSF TRs",   state.vsf_done);
    topic!(2, 164u8, b"AMWA NMOS", state.nmos_done);

    // Continue on a separate row, roughly centered
    print_text(window, font(), (84u8, 36u8), if cursor == 3 { pkstr!(b">") } else { pkstr!(b" ") });
    print_text(window, if cursor == 3 { blue_font() } else { font() }, (92u8, 36u8), pkstr!(b"Continue"));

    window.copy_to_vram();
}

pub async fn slide_hub(context: &Context<'_>, state: &HubState) -> HubChoice {
    // Legendary birds: each loaded with its own sprite index so palettes don't collide.
    //   Articuno (144) = ST 2110  |  Zapdos (145) = NMOS  |  Moltres (146) = VSF
    // Positioned at y=40 (¼ screen down), starting at x=30 (⅛ screen right),
    // evenly spaced so all three fit within the 240px screen width.
    let _articuno = poke_sprite_n(144, (40i16,  40i16), 1, 0); // ST 2110
    let _zapdos   = poke_sprite_n(145, (116i16, 40i16), 1, 1); // VSF
    let _moltres  = poke_sprite_n(146, (192i16, 40i16), 1, 2); // NMOS

    // Menu window just below the birds (birds bottom at y≈104, window starts at tile 13 = 104px)
    let window = create_msg_window(context, (0, 13, 30, 7), 0).await;
    let mut cursor = 0usize;
    redraw_hub_menu(&window, context, state, cursor);

    // Reveal (previous slide left screen black)
    graphics::fade_palette(PaletteMask::ALL, 5, 16, 0, 0).await;

    loop {
        sleep(1).await;

        // Left/Right navigate the top-row topics; Down/Up reach Continue
        let moved = if Button::Right.repeat() && cursor < 2 {
            cursor += 1; true
        } else if Button::Left.repeat() && cursor > 0 && cursor < 3 {
            cursor -= 1; true
        } else if Button::Down.repeat() && cursor < 3 {
            cursor = 3; true
        } else if Button::Up.repeat() && cursor == 3 {
            cursor = 0; true
        } else {
            false
        };

        if moved {
            redraw_hub_menu(&window, context, state, cursor);
        }

        if Button::A.pressed() {
            graphics::fade_palette(PaletteMask::ALL, 5, 0, 16, 0).await;
            window.clear_with_border();
            return match cursor {
                0 => HubChoice::ST2110,
                1 => HubChoice::VSF,
                2 => HubChoice::NMOS,
                _ => HubChoice::Continue,
            };
        }
    }
}

// ─── Content slides ───────────────────────────────────────────────────────────

pub async fn slide_intro(context: &Context<'_>) {
    let window = create_msg_window(context, (4, 7, 22, 8), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"What is IPMX ?"));
    let text = pkstr!(
        b"An introductory presentation
about open standards for
interoperable media over IP."
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(137, (200, 100), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_ipmx_what(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"What is IPMX ?"));
    let text = pkstr!(
        b"
A suite of open standards that let
systems trade audio, video and
metadata interoperably over IP.

Video can be raw (lossless) or
compressed (JPEG-XS, HEVC) for
flexibility between quality and
bandwidth efficiency.
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(474, (200, 40), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_st2110(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"SMPTE ST 2110"));
    let text = pkstr!(
        b"
The backbone of IPMX.

Defines how to send uncompressed
video, audio and data over IP.

The rules for media to travel
between devices, replacing
analog methods like SDI.
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(82, (200, 40), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_nmos(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"AMWA NMOS"));
    let text = pkstr!(
        b"
Networked Media Open Specifications.
Solves: how do you discover,
connect and manage streams?

Like a middleman advertising every
node, device, sender and receiver
so all your devices can find each
other automatically.
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(600, (200, 40), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_vsf(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Video Services Forum"));
    let text = pkstr!(
        b"
VSF provides Technical
Recommendations that fill the gaps.

Answers questions like:
- How much bandwidth do I need?
- How do I ensure timing?
- How to make it all work?
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(125, (200, 40), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_ipmx_value(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"What IPMX packages"));
    let text = pkstr!(
        b"
ST 2110 = The language
  How video and audio are sent

NMOS = The conversation
  How devices find each other

VSF TRs = The practical guide
  How to make it all work

IPMX bundles all of these with
certification requirements.
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(94, (200, 32), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_benefits(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"What IPMX enables"));
    let text = pkstr!(
        b"
Timing Control
  PTP syncs everything to the nanosecond.

Interoperability
  All certified devices speak the same rules.

Easy Setup
  Devices connect and work automatically.
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(243, (200, 32), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_example(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Real-world example"));
    let text = pkstr!(
        b"
In a new production facility:
- Cameras output ST 2110 streams
  discovered by NMOS automatically
- Router uses IS-05 to connect
  any source to any destination
- Multiviewer finds all sources
  via IS-04 without manual config
- Everything synced via PTP
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(202, (200, 32), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_benefits_2(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Benefits"));
    let text = pkstr!(
        b"
No manual cabling plans needed, like it
was needed with analog media. Making
the workflow for media workers a lot
simpler and easier!
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(52, (200, 124), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_why_now(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Why IPMX matters now"));
    let text = pkstr!(
        b"
The industry is at an
inflection point.

Old infrastructure is aging and
new demands (4K, 8K, HDR) require
rethinking our technologies.
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(31, (200, 32), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_why_now_2(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Why IPMX matters now"));
    let text = pkstr!(
        b"
IPMX provides a proven,
standardized path forward that
brings everyone under one
certification umbrella.
    "
    );
    print_text(&window, font(), (0, 14), text);
    let _sprite = poke_sprite(34, (200, 32), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_nmos_live(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 16), 0).await;
    let _sprite = poke_sprite(384, (200, 32), 1);

    print_text(&window, bigfont(), (0, 0), pkstr!(b"NMOS live demo"));
    print_text(&window, font(), (0, 16), pkstr!(b"You can even do stuff like this:"));

    // Fade in immediately so "waiting" is visible while polling
    graphics::fade_palette(PaletteMask::ALL, 5, 16, 0, 0).await;

    if !nmos_ready() {
        print_text(&window, font(), (0, 60), pkstr!(b"Waiting for NMOS bridge..."));
        while !nmos_ready() {
            sleep(1).await;
        }
        // Bridge is ready — clear the waiting text and redraw
        window.fill(1);
        window.draw_border(context.border_gfx);
        window.put_tilemap();
        window.copy_to_vram();
        sleep(1).await;
        print_text(&window, bigfont(), (0, 0), pkstr!(b"NMOS live demo"));
        print_text(&window, font(), (0, 16), pkstr!(b"You can even do stuff like this:"));
    }

    print_text(&window, blue_font(), (0, 34), pkstr!(b"Sender"));
    print_text(&window, font(), (0, 46), nmos_sender_label());
    print_text(&window, blue_font(), (0, 68), pkstr!(b"Receiver"));
    print_text(&window, font(), (0, 80), nmos_receiver_label());

    wait_a_button().await;
    graphics::fade_palette(PaletteMask::ALL, 5, 0, 16, 0).await;
    window.clear_with_border();
}

// ── Envelope sprite data (16×16, 4bpp) ───────────────────────────────────────
// 4 tiles arranged 2×2.  Palette: 0=transparent, 1=black outline, 2=white body.
const ENVELOPE_TILES: [u8; 128] = [
    // Tile 0: top-left — left half of the V-flap
    0x11, 0x11, 0x11, 0x11, // row 0: ████████
    0x21, 0x21, 0x22, 0x22, // row 1: █░█░░░░░
    0x21, 0x12, 0x22, 0x22, // row 2: █░░█░░░░
    0x21, 0x22, 0x21, 0x22, // row 3: █░░░█░░░
    0x21, 0x22, 0x12, 0x22, // row 4: █░░░░█░░
    0x21, 0x22, 0x22, 0x21, // row 5: █░░░░░█░
    0x21, 0x22, 0x22, 0x12, // row 6: █░░░░░░█
    0x21, 0x22, 0x22, 0x22, // row 7: █░░░░░░░
    // Tile 1: top-right — right half of the V-flap
    0x11, 0x11, 0x11, 0x11, // row 0: ████████
    0x22, 0x22, 0x12, 0x12, // row 1: ░░░░░█░█
    0x22, 0x22, 0x21, 0x12, // row 2: ░░░░█░░█
    0x22, 0x12, 0x22, 0x12, // row 3: ░░░█░░░█
    0x22, 0x21, 0x22, 0x12, // row 4: ░░█░░░░█
    0x12, 0x22, 0x22, 0x12, // row 5: ░█░░░░░█
    0x21, 0x22, 0x22, 0x12, // row 6: █░░░░░░█  ← centre seam
    0x22, 0x22, 0x22, 0x12, // row 7: ░░░░░░░█
    // Tile 2: bottom-left — left edge + body
    0x21, 0x22, 0x22, 0x22,
    0x21, 0x22, 0x22, 0x22,
    0x21, 0x22, 0x22, 0x22,
    0x21, 0x22, 0x22, 0x22,
    0x21, 0x22, 0x22, 0x22,
    0x21, 0x22, 0x22, 0x22,
    0x21, 0x22, 0x22, 0x22,
    0x11, 0x11, 0x11, 0x11, // bottom border
    // Tile 3: bottom-right — body + right edge
    0x22, 0x22, 0x22, 0x12,
    0x22, 0x22, 0x22, 0x12,
    0x22, 0x22, 0x22, 0x12,
    0x22, 0x22, 0x22, 0x12,
    0x22, 0x22, 0x22, 0x12,
    0x22, 0x22, 0x22, 0x12,
    0x22, 0x22, 0x22, 0x12,
    0x11, 0x11, 0x11, 0x11, // bottom border
];
const ENVELOPE_PAL: [u16; 16] = [
    0x0000, // 0: transparent
    0x0000, // 1: black outline
    0x7FFF, // 2: white body
    0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000,
];
const ENVELOPE_ANIM_SEQ: &[AnimCmd] = &[anim_frame(0, 255, false, false), anim_end()];
const ENVELOPE_ANIM_TABLE: &[*const AnimCmd] =
    &[ENVELOPE_ANIM_SEQ.as_ptr(), core::ptr::null()];

pub async fn slide_anim_generated(context: &Context<'_>) {
    // Text box at the top (5 tiles = 40px, fits title + one subtitle line)
    let window = create_msg_window(context, (2, 2, 26, 5), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Another use case?"));
    print_text(&window, font(), (0, 14), pkstr!(b"So we can make cool stuff like this as well."));

    // Nidoran M on the left (sender), Nidoran F on the right (receiver).
    // Each on its own OBJ palette slot so colours don't bleed into each other.
    let _sender   = poke_sprite_n(32, ( 28i16, 100i16), 1, 0);
    let _receiver = poke_sprite_n(29, (208i16, 100i16), 1, 2);

    // Envelope sprite on OBJ palette slot 1 (won't collide with Nidorino).
    let tile_buf: AllocBuf<TileBitmap4bpp> = AllocBuf::new({
        let b: Box<[u8]> = Box::new(ENVELOPE_TILES);
        b
    });
    let env_palette = load_obj_palette(1, &ENVELOPE_PAL);
    let env_sheet = SpriteSheet::load(tile_buf, 0x1300u16, SPRITE_SIZE_16x16 as u8);
    let env_anims = SpriteAnims {
        anims: ENVELOPE_ANIM_TABLE.as_ptr(),
        affine_anims: DUMMY_SPRITE_ANIMS.affine_anims,
    };
    let letter = SheetSprite::load(&env_sheet, env_anims, env_palette);
    letter.set_pos(Vec2D::new(56i16, 100i16));
    letter.start_animation(0);

    graphics::fade_palette(PaletteMask::ALL, 5, 16, 0, 0).await;

    // Fly the envelope from Nidoran M to Nidoran F
    let mut x: i16 = 56;
    while x <= 179 {
        letter.set_pos(Vec2D::new(x, 100i16));
        sleep(1).await;
        x += 2;
    }

    // Envelope has landed — signal the bridge to connect sender → receiver
    if nmos_ready() {
        nmos_request_connect();
    }

    wait_a_button().await;

    graphics::fade_palette(PaletteMask::ALL, 5, 0, 16, 0).await;
    drop(letter);
    drop(env_sheet);
    drop(_sender);
    drop(_receiver);
    window.clear_with_border();
}

include_res_lz!(
    SLIDE_STATUS_GFX,
    "../../graphics/party_menu_full/status_icons.4bpp"
);
include_res_lz!(
    SLIDE_STATUS_PAL,
    "../../graphics/party_menu_full/status_icons.gbapal"
);

const STATUS_CYCLE_SEQ: &[AnimCmd] = &[
    anim_frame(0, 40, false, false),
    anim_frame(4, 40, false, false),
    anim_frame(8, 40, false, false),
    anim_frame(12, 40, false, false),
    anim_frame(16, 40, false, false),
    anim_frame(20, 40, false, false),
    anim_end(),
];
const STATUS_CYCLE_TABLE: &[*const AnimCmd] = &[STATUS_CYCLE_SEQ.as_ptr(), core::ptr::null()];

pub async fn slide_anim_imported(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 4), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Imported animation"));
    print_text(
        &window,
        font(),
        (0, 14),
        pkstr!(b"Engine AnimCmd frame table"),
    );

    let gfx = SLIDE_STATUS_GFX.load();
    let pal_data = SLIDE_STATUS_PAL.load();
    let palette = load_obj_palette(0, &pal_data.get());
    let sheet = SpriteSheet::load(gfx, 0x1235, 0b0100);
    let anims = SpriteAnims {
        anims: STATUS_CYCLE_TABLE.as_ptr(),
        affine_anims: DUMMY_SPRITE_ANIMS.affine_anims,
    };
    let sprite = SheetSprite::load(&sheet, anims, palette);
    sprite.set_pos(Vec2D::new(120, 80));
    sprite.start_animation(0);

    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_thank_you(context: &Context<'_>) {
    let window = create_msg_window(context, (8, 7, 14, 6), 0).await;

    let text = pkstr!(
        b"That's it !
Thank you for your
attention !"
    );
    print_text(&window, bigfont(), (0, 0), text);
    let _sprite = poke_sprite(155, (200, 80), 2);
    transition_slide().await;
    window.clear_with_border();
}
