use bindings::future::sleep;
use bindings::graphics::{SpriteSheet, Vec2D, *};
use bindings::pokeemerald::*;
use bindings::resources::Buffer;
use bindings::{include_res_lz, pkstr};

use super::nmos::*;
use super::ui::*;

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
    print_text(&window, bigfont(), (0, 0), pkstr!(b"NMOS live demo"));
    print_text(
        &window,
        font(),
        (0, 16),
        pkstr!(b"You can even do stuff like this:"),
    );

    if !nmos_ready() {
        print_text(
            &window,
            font(),
            (0, 60),
            pkstr!(b"Waiting for NMOS bridge..."),
        );
    } else {
        print_text(&window, blue_font(), (0, 34), pkstr!(b"Sender"));
        print_text(&window, font(), (0, 46), nmos_sender_label());
        print_text(&window, blue_font(), (0, 68), pkstr!(b"Receiver"));
        print_text(&window, font(), (0, 80), nmos_receiver_label());

        let status = match nmos_status() {
            0 => pkstr!(b"Idle"),
            1 => pkstr!(b"Connecting..."),
            2 => pkstr!(b"Connected !"),
            3 => pkstr!(b"Failed"),
            _ => pkstr!(b"Unknown"),
        };
        print_text(&window, font(), (0, 106), pkstr!(b"Status:"));
        print_text(&window, font(), (50, 106), status);
    }

    let _sprite = poke_sprite(384, (200, 32), 1);
    transition_slide().await;
    window.clear_with_border();
}

pub async fn slide_anim_generated(context: &Context<'_>) {
    let window = create_msg_window(context, (2, 2, 26, 4), 0).await;
    print_text(&window, bigfont(), (0, 0), pkstr!(b"Another use case?"));
    print_text(
        &window,
        font(),
        (0, 14),
        pkstr!(b"So we can make cool stuff like this as well."),
    );

    // TODO: make animation of pokemon holding frame with changing video

    let mut sprite = poke_sprite(25, (40, 80), 1);
    let mut x: i16 = 40;
    let mut dx: i16 = 2;
    for _ in 0..180u32 {
        if x >= 200 || x <= 40 {
            dx = -dx;
        }
        x += dx;
        sprite.handle().set_pos(Vec2D::new(x, 80));
        sleep(1).await;
    }
    transition_slide().await;
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
