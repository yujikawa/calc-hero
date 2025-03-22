use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    // キャラ画像を読み込む
    let player_texture = load_texture("student.png").await.unwrap();
    let background_texture = load_texture("school_bg.png").await.unwrap(); // 背景画像を読み込み
    player_texture.set_filter(FilterMode::Nearest); // ドット感を保つ設定（必要なら）

    let mut bg_y = 0.0;
    let scroll_speed = 100.0;
    // プレイヤーを画面中央に固定（参照渡し）
    let scale = 0.5;
    let player_width = player_texture.width() * scale;
    let player_height = player_texture.height() * scale;
    let mut player_x = screen_width() / 2.0 - player_width / 2.0;
    let player_y = screen_height() - player_height;

    loop {
        let dt = get_frame_time();
        bg_y += scroll_speed * dt;

        if bg_y >= background_texture.height() {
            bg_y -= background_texture.height();
        }

        // 背景を2枚描画（参照渡し）
        let bg_x = screen_width() / 2.0 - background_texture.width() / 2.0;
        draw_texture(&background_texture, bg_x, bg_y, WHITE);
        draw_texture(
            &background_texture,
            bg_x,
            bg_y - background_texture.height(),
            WHITE,
        );

        // プレイヤーの描画
        let move_speed = 200.0;
        if is_key_down(KeyCode::Left) {
            player_x -= move_speed * dt;
        }
        if is_key_down(KeyCode::Right) {
            player_x += move_speed * dt;
        }

        // 画面外に出ないように制限
        let bg_x = screen_width() / 2.0 - background_texture.width() / 2.0;

        let min_x = bg_x;
        let max_x = bg_x + background_texture.width() - player_width;

        player_x = player_x.clamp(min_x, max_x);

        draw_texture_ex(
            &player_texture,
            player_x,
            player_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    player_width,  // ← 横幅を 50% に
                    player_height, // ← 高さも 50%
                )),
                ..Default::default()
            },
        );

        // 数字パネル
        let translucent_gray = Color::new(0.2, 0.2, 0.2, 0.6);

        // 仮の数字パネル
        let left_value = 12;
        let right_value = 24;

        // パネルサイズ
        let panel_width = 300.0;
        let panel_height = 120.0;

        // Y位置（プレイヤーのちょい前くらい）
        let panel_y = screen_height() / 2.0 - player_height / 2.0 - 80.0;

        // 左パネル
        let left_x = screen_width() / 3.0 - panel_width / 2.0;
        draw_rectangle(
            left_x,
            bg_y - panel_y,
            panel_width,
            panel_height,
            translucent_gray,
        );
        draw_text(
            &left_value.to_string(),
            left_x + 100.0,
            bg_y - panel_y + 80.0,
            100.0,
            WHITE,
        );

        // 右パネル
        let right_x = screen_width() * 3.0 / 4.5 - panel_width / 2.0;
        draw_rectangle(
            right_x,
            bg_y - panel_y,
            panel_width,
            panel_height,
            translucent_gray,
        );
        draw_text(
            &right_value.to_string(),
            right_x + 100.0,
            bg_y - panel_y + 80.0,
            100.0,
            WHITE,
        );

        // お題パネル
        // お題
        let question_text = "? × 3 = 12";

        // パネルサイズ
        let panel_width = 400.0;
        let panel_height = 100.0;

        // 表示位置（中央上部）
        let panel_x = screen_width() / 2.0 - panel_width / 2.0;
        let panel_y = 30.0;

        // 黒い背景（不透明 or 透過）
        let panel_color = Color::new(0.0, 0.0, 0.0, 0.8); // ← 0.8で少し透過

        // 背景パネル
        draw_rectangle(panel_x, panel_y, panel_width, panel_height, panel_color);

        // テキストを中央に表示
        let text_size = 80.0;
        let text_metrics = measure_text(question_text, None, text_size as u16, 1.0);
        let text_x = panel_x + (panel_width - text_metrics.width) / 2.0;
        let text_y = panel_y + panel_height / 2.0 + text_metrics.height / 2.5;

        draw_text(question_text, text_x, text_y, text_size, WHITE);
        next_frame().await;
    }
}
