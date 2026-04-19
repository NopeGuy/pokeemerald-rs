use bindings::charmap::Pkstr;
use bindings::future::sleep;
use bindings::graphics::{self, BgHandle, TilesetHandle, Window, *};
use bindings::input::Button;
use bindings::pokeemerald::*;
use bindings::pkstr;

pub struct Context<'a> {
    pub bg: BgHandle<'a>,
    pub fg: BgHandle<'a>,
    pub msg_box: TilesetHandle,
    pub border_gfx: TilesetHandle,
}

pub async fn clear_ui() {
    unsafe {
        SetVBlankHBlankCallbacksToNull();
        ResetVramOamAndBgCntRegs();
        ClearScheduledBgCopiesToVram();
        sleep(1).await;

        ResetPaletteFade();
        sleep(1).await;

        ResetSpriteData();
        sleep(1).await;

        FreeAllSpritePalettes();
        sleep(1).await;

        Window::clear_all();
        sleep(1).await;

        ResetBgsAndClearDma3BusyFlags(0);
    }
}

pub async fn create_msg_window(
    context: &Context<'_>,
    rect: impl Into<Rect<u8>>,
    offset: u16,
) -> Window {
    let window = Window::create(
        context.fg,
        rect.into(),
        context.msg_box.palette,
        0x80 + offset,
    );
    sleep(1).await;
    window.fill(1);
    sleep(1).await;
    window.draw_border(context.border_gfx);
    window.put_tilemap();
    window.copy_to_vram();
    sleep(1).await;
    window
}

pub async fn wait_a_button() {
    while !Button::A.pressed() {
        sleep(1).await
    }
}

pub async fn transition_slide() {
    graphics::fade_palette(PaletteMask::ALL, 5, 16, 0, 0).await;
    wait_a_button().await;
    graphics::fade_palette(PaletteMask::ALL, 5, 0, 16, 0).await;
}

pub fn font() -> Font {
    Font::new(FONT_SMALL as u8)
}

pub fn bigfont() -> Font {
    Font {
        fg_color: 4,
        shadow_color: 5,
        ..Font::new(FONT_NORMAL as u8)
    }
}

pub fn blue_font() -> Font {
    Font {
        fg_color: 8,
        shadow_color: 9,
        ..Font::new(FONT_SMALL as u8)
    }
}

pub fn poke_sprite(species: u16, pos: impl Into<Vec2D<i16>>, priority: u8) -> PokemonSpritePic {
    poke_sprite_n(species, pos, priority, 0)
}

pub fn poke_sprite_n(
    species: u16,
    pos: impl Into<Vec2D<i16>>,
    priority: u8,
    index: u8,
) -> PokemonSpritePic {
    let mut sprite = PokemonSpritePic::new_by_index(species, index);
    sprite.handle().set_priority(priority);
    sprite.handle().set_pos(pos.into());
    sprite
}

pub fn print_text(window: &Window, font: Font, pos: impl Into<Vec2D<u8>>, text: &Pkstr) {
    window.print_text(text, pos.into(), font);
}
