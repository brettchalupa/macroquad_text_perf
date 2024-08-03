use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Text Perf".to_owned(),
        fullscreen: false,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("./DejaVuSansMono.ttf").await.unwrap();
    let mut positions = vec![];

    for _ in 1..3501 {
        positions.push((rand::gen_range(64., 1200.), rand::gen_range(64., 650.)))
    }
    loop {
        clear_background(BLACK);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 16., 32., 32., WHITE);
        draw_text(
            format!("texts drawn: {}", positions.len()).as_str(),
            128.,
            32.,
            32.,
            WHITE,
        );
        println!("{}", get_fps());

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for (i, (x, y)) in positions.iter_mut().enumerate() {
            draw_text_ex(
                format!("{}: This is some text", i).as_str(),
                *x,
                *y,
                TextParams {
                    font: Some(&font),
                    font_size: 16,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }

        next_frame().await
    }
}
