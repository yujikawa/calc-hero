use macroquad::rand::srand;
use macroquad::time::get_time;
use macroquad::{prelude::*, rand::gen_range};

#[macroquad::main("BasicShapes")]
async fn main() {
    // キャラ画像を読み込む
    srand(get_time() as u64);
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

    // 数式
    let mut a = gen_range(1, 100); // 1〜99
    let mut b = gen_range(1, 100); // 1〜9
    let mut correct_on_left = gen_range(0, 2) == 0; // 0: left 1: right
    let mut noise = gen_range(1, 5);
    let mut correct_ans = a * b;
    let mut incorrect_ans = a * b + noise;
    let mut question_text = format!("{} × {} = ?", a, b);
    let (mut left_value, mut right_value) = if correct_on_left {
        (correct_ans, incorrect_ans)
    } else {
        (incorrect_ans, correct_ans)
    };
    // 正解判定
    let mut selection_made = false;
    let mut is_correct = false;
    let mut selection_timer = 0.0;

    loop {
        let dt = get_frame_time();
        // bg_y += scroll_speed * dt;

        // if bg_y >= background_texture.height() {
        //     bg_y -= background_texture.height();
        // }

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
        // let move_speed = 200.0;
        // if is_key_down(KeyCode::Left) {
        //     player_x -= move_speed * dt;
        // }
        // if is_key_down(KeyCode::Right) {
        //     player_x += move_speed * dt;
        // }

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

        // パネルサイズ
        let panel_width = 300.0;
        let panel_height = 120.0;

        // Y位置（プレイヤーのちょい前くらい）
        let panel_y = screen_height() / 1.0 - player_height / 2.0 - 80.0;

        // 左パネル
        let left_x = screen_width() / 3.0 - panel_width / 2.0;
        draw_rectangle(left_x, panel_y, panel_width, panel_height, translucent_gray);
        draw_text(
            &left_value.to_string(),
            left_x + 100.0,
            panel_y + 80.0,
            100.0,
            WHITE,
        );

        // 右パネル
        let right_x = screen_width() * 3.0 / 4.5 - panel_width / 2.0;
        draw_rectangle(
            right_x,
            panel_y,
            panel_width,
            panel_height,
            translucent_gray,
        );
        draw_text(
            &right_value.to_string(),
            right_x + 100.0,
            panel_y + 80.0,
            100.0,
            WHITE,
        );

        // お題パネル
        // お題

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
        let text_metrics = measure_text(&question_text, None, text_size as u16, 1.0);
        let text_x = panel_x + (panel_width - text_metrics.width) / 2.0;
        let text_y = panel_y + panel_height / 2.0 + text_metrics.height / 2.5;

        draw_text(&question_text, text_x, text_y, text_size, WHITE);

        // 正解チェック
        if !selection_made {
            if is_key_pressed(KeyCode::Left) {
                selection_made = true;
                is_correct = correct_on_left;
                selection_timer = 0.0; // タイマー開始
            } else if is_key_pressed(KeyCode::Right) {
                selection_made = true;
                is_correct = !correct_on_left;
                selection_timer = 0.0; // タイマー開始
            }
        } else {
            selection_timer += dt;
            if selection_timer >= 2.0 {
                selection_made = false;
                selection_timer = 0.0;

                a = gen_range(1, 100); // 1〜9
                b = gen_range(1, 100); // 1〜9
                correct_on_left = gen_range(0, 2) == 0; // 0: left 1: right
                noise = gen_range(1, 5);
                correct_ans = a * b;
                incorrect_ans = a * b + noise;

                question_text = format!("{} × {} = ?", a, b);
                (left_value, right_value) = if correct_on_left {
                    (correct_ans, incorrect_ans)
                } else {
                    (incorrect_ans, correct_ans)
                };
            }
        }

        if selection_made {
            let result_text = if is_correct { "Correct!" } else { "Incorrect" };
            let text_size = 100.0;
            let text_metrics = measure_text(result_text, None, text_size as u16, 1.0);
            let text_x = screen_width() / 2.0 - text_metrics.width / 2.0;
            let text_y = screen_height() / 2.0;
            let color = if is_correct { GREEN } else { RED };

            draw_text(result_text, text_x, text_y, text_size, color);
        }
        next_frame().await;
    }
}
