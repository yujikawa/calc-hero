use macroquad::rand::srand;
use macroquad::time::get_time;
use macroquad::{prelude::*, rand::gen_range};

enum GameState {
    StartMenu,
    Playing(OperationMode),
}

#[derive(Clone, Copy)]
enum OperationMode {
    Addition,
    Multiplication,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    // キャラ画像を読み込む
    srand(get_time() as u64);
    let background_texture = load_texture("school_bg.png").await.unwrap(); // 背景画像を読み込み

    // 背景の設定
    let bg_y = 0.0;
    let bg_x = screen_width() / 2.0 - background_texture.width() / 2.0;

    // 数式
    let mut a;
    let mut b;
    let mut correct_on_left = false; // 0: left 1: right
    let mut noise: i32;
    let mut correct_ans;
    let mut incorrect_ans;
    let mut question_text = format!("a + or x b = ?");

    let (mut left_value, mut right_value) = (0, 0);

    // 正解判定
    let mut selection_made = false;
    let mut is_correct = false;
    let mut selection_timer = 0.0;

    let mut game_state = GameState::StartMenu;

    loop {
        clear_background(BLACK);
        draw_texture(&background_texture, bg_x, bg_y, WHITE);
        match game_state {
            GameState::StartMenu => {
                draw_text("CALC HERO", 100.0, 100.0, 40.0, WHITE);

                let add_button = Rect::new(100.0, 200.0, 300.0, 60.0);
                let mul_button = Rect::new(100.0, 300.0, 300.0, 60.0);

                draw_rectangle(
                    add_button.x,
                    add_button.y,
                    add_button.w,
                    add_button.h,
                    GREEN,
                );
                draw_rectangle(
                    mul_button.x,
                    mul_button.y,
                    mul_button.w,
                    mul_button.h,
                    YELLOW,
                );

                draw_text(
                    "Addition",
                    add_button.x + 20.0,
                    add_button.y + 40.0,
                    30.0,
                    BLACK,
                );
                draw_text(
                    "Multiplication",
                    mul_button.x + 20.0,
                    mul_button.y + 40.0,
                    30.0,
                    BLACK,
                );

                if is_mouse_button_pressed(MouseButton::Left) {
                    let mouse_pos = mouse_position().into();
                    if add_button.contains(mouse_pos) {
                        game_state = GameState::Playing(OperationMode::Addition);
                        a = gen_range(1, 10); // 1〜99
                        b = gen_range(1, 10); // 1〜9
                        correct_on_left = gen_range(0, 2) == 0; // 0: left 1: right
                        let noise: i32 = gen_range(1, 5);
                        correct_ans = a + b;
                        incorrect_ans = a + b + noise;
                        question_text = format!("{} + {} = ?", a, b);

                        (left_value, right_value) = if correct_on_left {
                            (correct_ans, incorrect_ans)
                        } else {
                            (incorrect_ans, correct_ans)
                        };
                    } else if mul_button.contains(mouse_pos) {
                        game_state = GameState::Playing(OperationMode::Multiplication);
                        a = gen_range(1, 100); // 1〜99
                        b = gen_range(1, 100); // 1〜9
                        correct_on_left = gen_range(0, 2) == 0; // 0: left 1: right
                        let noise: i32 = gen_range(1, 5);
                        correct_ans = a * b;
                        incorrect_ans = a * b + noise;
                        question_text = format!("{} x {} = ?", a, b);

                        (left_value, right_value) = if correct_on_left {
                            (correct_ans, incorrect_ans)
                        } else {
                            (incorrect_ans, correct_ans)
                        };
                    }
                }
            }
            GameState::Playing(mode) => match mode {
                OperationMode::Addition => {
                    let dt = get_frame_time();

                    // 背景を描画
                    draw_texture(&background_texture, bg_x, bg_y, WHITE);

                    // 数字パネル
                    let translucent_gray = Color::new(0.2, 0.2, 0.2, 0.6);

                    // パネルサイズ
                    let answer_panel_width = 300.0;
                    let answer_panel_height = 120.0;

                    // Y位置（プレイヤーのちょい前くらい）
                    let answer_panel_y = screen_height() / 2.0;

                    // 左パネル
                    let left_panel_x = screen_width() / 3.0 - answer_panel_width / 2.0;
                    draw_rectangle(
                        left_panel_x,
                        answer_panel_y,
                        answer_panel_width,
                        answer_panel_height,
                        translucent_gray,
                    );
                    draw_text(
                        &left_value.to_string(),
                        left_panel_x + 100.0,
                        answer_panel_y + 80.0,
                        100.0,
                        WHITE,
                    );

                    // 右パネル
                    let right_panel_x = screen_width() * 3.0 / 4.5 - answer_panel_width / 2.0;
                    draw_rectangle(
                        right_panel_x,
                        answer_panel_y,
                        answer_panel_width,
                        answer_panel_height,
                        translucent_gray,
                    );
                    draw_text(
                        &right_value.to_string(),
                        right_panel_x + 100.0,
                        answer_panel_y + 80.0,
                        100.0,
                        WHITE,
                    );

                    // お題パネル
                    // パネルサイズ
                    let question_panel_width = 400.0;
                    let question_panel_height = 100.0;

                    // 表示位置（中央上部）
                    let question_panel_x = screen_width() / 2.0 - question_panel_width / 2.0;
                    let question_panel_y = 30.0;

                    // 黒い背景（不透明 or 透過）
                    let panel_color = Color::new(0.0, 0.0, 0.0, 0.8); // ← 0.8で少し透過

                    // 背景パネル
                    draw_rectangle(
                        question_panel_x,
                        question_panel_y,
                        question_panel_width,
                        question_panel_height,
                        panel_color,
                    );

                    // テキストを中央に表示
                    let text_size = 80.0;
                    let text_metrics = measure_text(&question_text, None, text_size as u16, 1.0);
                    let text_x =
                        question_panel_x + (question_panel_width - text_metrics.width) / 2.0;
                    let text_y =
                        question_panel_y + question_panel_height / 2.0 + text_metrics.height / 2.5;

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

                        if is_mouse_button_pressed(MouseButton::Left) {
                            let (mx, my) = mouse_position();
                            // 左のパネルが押されたか判定
                            if mx >= left_panel_x
                                && mx <= left_panel_x + answer_panel_width
                                && my >= answer_panel_y
                                && my <= answer_panel_y + answer_panel_height
                            {
                                selection_made = true;
                                is_correct = correct_on_left;
                                selection_timer = 0.0;
                            }

                            // 右のパネルが押されたか判定
                            if mx >= right_panel_x
                                && mx <= right_panel_x + answer_panel_width
                                && my >= answer_panel_y
                                && my <= answer_panel_y + answer_panel_height
                            {
                                selection_made = true;
                                is_correct = !correct_on_left;
                                selection_timer = 0.0;
                            }
                        }
                    } else {
                        selection_timer += dt;
                        if selection_timer >= 2.0 {
                            srand(get_time() as u64);
                            selection_made = false;
                            selection_timer = 0.0;

                            a = gen_range(1, 10); // 1〜9
                            b = gen_range(1, 10); // 1〜9
                            correct_on_left = gen_range(0, 2) == 0; // 0: left 1: right
                            noise = gen_range(1, 5);
                            correct_ans = a + b;
                            incorrect_ans = a + b + noise;

                            question_text = format!("{} + {} = ?", a, b);
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
                }
                OperationMode::Multiplication => {
                    let dt = get_frame_time();

                    // 背景を描画
                    draw_texture(&background_texture, bg_x, bg_y, WHITE);

                    // 数字パネル
                    let translucent_gray = Color::new(0.2, 0.2, 0.2, 0.6);

                    // パネルサイズ
                    let answer_panel_width = 300.0;
                    let answer_panel_height = 120.0;

                    // Y位置（プレイヤーのちょい前くらい）
                    let answer_panel_y = screen_height() / 2.0;

                    // 左パネル
                    let left_panel_x = screen_width() / 3.0 - answer_panel_width / 2.0;
                    draw_rectangle(
                        left_panel_x,
                        answer_panel_y,
                        answer_panel_width,
                        answer_panel_height,
                        translucent_gray,
                    );
                    draw_text(
                        &left_value.to_string(),
                        left_panel_x + 100.0,
                        answer_panel_y + 80.0,
                        100.0,
                        WHITE,
                    );

                    // 右パネル
                    let right_panel_x = screen_width() * 3.0 / 4.5 - answer_panel_width / 2.0;
                    draw_rectangle(
                        right_panel_x,
                        answer_panel_y,
                        answer_panel_width,
                        answer_panel_height,
                        translucent_gray,
                    );
                    draw_text(
                        &right_value.to_string(),
                        right_panel_x + 100.0,
                        answer_panel_y + 80.0,
                        100.0,
                        WHITE,
                    );

                    // お題パネル
                    // パネルサイズ
                    let question_panel_width = 400.0;
                    let question_panel_height = 100.0;

                    // 表示位置（中央上部）
                    let question_panel_x = screen_width() / 2.0 - question_panel_width / 2.0;
                    let question_panel_y = 30.0;

                    // 黒い背景（不透明 or 透過）
                    let panel_color = Color::new(0.0, 0.0, 0.0, 0.8); // ← 0.8で少し透過

                    // 背景パネル
                    draw_rectangle(
                        question_panel_x,
                        question_panel_y,
                        question_panel_width,
                        question_panel_height,
                        panel_color,
                    );

                    // テキストを中央に表示
                    let text_size = 80.0;
                    let text_metrics = measure_text(&question_text, None, text_size as u16, 1.0);
                    let text_x =
                        question_panel_x + (question_panel_width - text_metrics.width) / 2.0;
                    let text_y =
                        question_panel_y + question_panel_height / 2.0 + text_metrics.height / 2.5;

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

                        if is_mouse_button_pressed(MouseButton::Left) {
                            let (mx, my) = mouse_position();
                            // 左のパネルが押されたか判定
                            if mx >= left_panel_x
                                && mx <= left_panel_x + answer_panel_width
                                && my >= answer_panel_y
                                && my <= answer_panel_y + answer_panel_height
                            {
                                selection_made = true;
                                is_correct = correct_on_left;
                                selection_timer = 0.0;
                            }

                            // 右のパネルが押されたか判定
                            if mx >= right_panel_x
                                && mx <= right_panel_x + answer_panel_width
                                && my >= answer_panel_y
                                && my <= answer_panel_y + answer_panel_height
                            {
                                selection_made = true;
                                is_correct = !correct_on_left;
                                selection_timer = 0.0;
                            }
                        }
                    } else {
                        selection_timer += dt;
                        if selection_timer >= 2.0 {
                            srand(get_time() as u64);
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
                }
            },
        }

        next_frame().await;
    }
}
