use eframe::egui::{Pos2, Rect};

pub struct GuiTools {}

impl GuiTools {
    /// 文字列の表示サイズを取得
    pub fn get_display_size(ctx: &egui::Context, ui: &egui::Ui, text: &str) -> egui::Vec2 {
        // レイアウトを作成してサイズを取得
        let font_id = egui::TextStyle::Body.resolve(&ctx.style());
        //let fill_color = ctx.style().visuals.widgets.noninteractive.bg_fill;
        let fill_color = ctx.style().visuals.widgets.active.bg_fill;
        let title_dummy = ui
            .painter()
            .layout_no_wrap(text.to_owned(), font_id.clone(), fill_color);

        title_dummy.size()
    }

    /// タイトル付きのフレームを表示
    pub fn show_titled_frame(
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        frame_title: &str,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
        let frame_response = egui::Frame::group(ui.style())
            .corner_radius(
                egui::CornerRadius::same(2), // 角の丸み
            )
            .show(ui, add_contents);

        let fill_color = ctx.style().visuals.panel_fill;
        let text_color = ctx.style().visuals.text_color();
        let font_id = egui::TextStyle::Body.resolve(&ctx.style());

        //////////////////////////
        // タイトル表示サイズを取得

        // レイアウトを作成してサイズを取得
        let title_dummy =
            ui.painter()
                .layout_no_wrap(frame_title.to_owned(), font_id.clone(), fill_color);

        let title_width = title_dummy.size().x;
        let title_height = title_dummy.size().y;

        // タイトルの背景を塗りつぶす
        let frame_rect = frame_response.response.rect;
        let left_top = frame_rect.left_top();
        ui.painter().rect_filled(
            Rect {
                min: Pos2 {
                    x: left_top.x + 5.0,
                    y: left_top.y - (title_height / 2.0),
                },
                max: Pos2 {
                    x: left_top.x + title_width + 5.0 + 5.0 + 5.0,
                    y: left_top.y + (title_height / 2.0),
                },
            },
            egui::CornerRadius::same(0), // 角の丸み(タイトルの背景の角の丸めは意味がない)
            fill_color,                  // 塗りつぶしの色
        );
        ui.painter().text(
            Pos2 {
                x: left_top.x + 5.0 + 5.0,
                y: left_top.y - (title_height / 2.0),
            },
            egui::Align2::LEFT_TOP, // 配置の基準点
            frame_title,            // 表示文字列
            font_id.clone(),        // フォント
            text_color,             // 文字色
        );
    }
}
