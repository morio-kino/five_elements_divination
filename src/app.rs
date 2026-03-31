use chrono::{DateTime, Datelike as _, Local, Timelike as _};
use eframe::egui::{Pos2, Rect};
use egui::Color32;
use egui::DragValue;
use egui::RichText;
use egui::{FontData, FontDefinitions, FontFamily};
use egui_dialogs::Dialogs;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::sync::Arc;
use std::time::Duration;
use strum::IntoEnumIterator as _;
use strum_macros::{Display, EnumIter};

mod gui_tools;
use gui_tools::GuiTools;

mod text_tools;
use text_tools::TextTools;

// 機能名の一覧
#[derive(Debug, EnumIter, FromPrimitive, Display)]
enum FuncNames {
    Application,       // アプリの画面表示
    Button,            // ボタン
    Checbox,           // チェックボックス
    Collapse,          // 折りたたみ
    Colmuns,           // 列分割
    Combobox,          // コンボボックス
    DragValue,         // 数値入力
    FirstSample,       // 最初のサンプル
    Frames,            // フレーム
    Grid,              // グリッド
    HorVer,            // 水平・垂直配置
    HorWrapping,       // 水平ラッピング
    Hyperlink,         // ハイパーリンク
    Image,             // 画像
    Layout,            // レイアウト（左寄せ、右寄せ、中央寄せ）
    Link,              // リンク
    MenuButton,        // メニューバー
    Panels,            //クライアント領域のパネル分割
    ProgressBar,       // プログレスバー
    RadioButton,       // ラジオボタン
    Scroll,            // スクロール
    Selectables,       // 選択ラベル、選択値
    Slider,            // スライダー
    SpaceAndSeparator, //余白・区切り・インデント
    Spinner,           // スピナー
    TextInput,         // テキスト入力
    Texts,             // テキスト表示
}

// 十干
#[derive(Clone, Copy, PartialEq, EnumIter, FromPrimitive, Display, Debug)]
#[repr(u8)]
enum TenCelestialStems {
    None,      // (未選択)
    YangWood,  // 甲,
    YinWood,   // 乙,
    YangFire,  // 丙
    YinFire,   // 丁
    YangEarth, // 戊
    YinEarth,  // 己
    YangMetal, // 庚
    YinMetal,  // 辛
    YangWater, // 壬
    YinWater,  // 癸
}
impl TenCelestialStems {
    fn as_char(&self) -> char {
        match &self {
            Self::None => '　',
            Self::YangWood => '甲',
            Self::YinWood => '乙',
            Self::YangFire => '丙',
            Self::YinFire => '丁',
            Self::YangEarth => '戊',
            Self::YinEarth => '己',
            Self::YangMetal => '庚',
            Self::YinMetal => '辛',
            Self::YangWater => '壬',
            Self::YinWater => '癸',
        }
    }
    fn as_string(&self) -> String {
        self.as_char().to_string()
    }
}

// 十二支
#[derive(Clone, Copy, PartialEq, EnumIter, FromPrimitive, Display, Debug)]
#[repr(u8)]
enum TwelveEarthlyBranches {
    None,    // (未選択)
    Rat,     // 子
    Ox,      // 丑
    Tiger,   // 寅
    Rabbit,  // 卯
    Dragon,  // 辰
    Snake,   // 巳
    Horse,   // 午
    Goat,    // 未
    Monkey,  // 申
    Rooster, // 酉
    Dog,     // 戌
    Pig,     // 亥
}
impl TwelveEarthlyBranches {
    fn as_char(&self) -> char {
        match &self {
            Self::None => '　',
            Self::Rat => '子',
            Self::Ox => '丑',
            Self::Tiger => '寅',
            Self::Rabbit => '卯',
            Self::Dragon => '辰',
            Self::Snake => '巳',
            Self::Horse => '午',
            Self::Goat => '未',
            Self::Monkey => '申',
            Self::Rooster => '酉',
            Self::Dog => '戌',
            Self::Pig => '亥',
        }
    }
    fn as_string(&self) -> String {
        self.as_char().to_string()
    }
}

enum VisualMode {
    Dark,
    Light,
    System,
}

pub struct WinSize {
    pub x: f32,
    pub y: f32,
}

pub const WIN_SIZE_SMALL: WinSize = WinSize { x: 800.0, y: 600.0 };
pub const WIN_SIZE_LARGE: WinSize = WinSize {
    x: 1600.0,
    y: 1200.0,
};

// ネイティブアプリの場合のサイズ変更処理
#[cfg(not(target_arch = "wasm32"))]
fn change_canvas_size(_ctx: &egui::Context, x: f32, y: f32) {
    _ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(x, y)));
}

// Webアプリの場合のサイズ変更処理
#[cfg(target_arch = "wasm32")]
fn change_canvas_size(_ctx: &egui::Context, x: f32, y: f32) {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("should hav a document on window");
    let canvas = document
        .get_element_by_id("the_canvas_id")
        .expect("body should hav a id=the_canvas_id");
    let x_str: String = x.to_string();
    let y_str: String = y.to_string();
    let x_ref: &str = &x_str;
    let y_ref: &str = &y_str;
    let width_result = canvas.set_attribute("width", x_ref);
    if width_result.is_err() {
        println!("canvas.set_attrigule(width) error {:?}", width_result);
    }
    let height_result = canvas.set_attribute("height", y_ref);
    if height_result.is_err() {
        println!("canvas.set_attrigule(height) error {:?}", height_result);
    }
}

/////////////////////////////////////
/// アプリのstruct
//#[derive(Default)]
pub struct DivinationApp<'a> {
    dialogs: Dialogs<'a>,

    use_now: bool,
    detail: bool,
    visual_mode: VisualMode,
    yyyy_mm_dd: String,
    hh_mm_ss: String,
    etoreki: String,

    nen_ten: TenCelestialStems,
    nen_twelve: TwelveEarthlyBranches,
    tsuki_ten: TenCelestialStems,
    tsuki_twelve: TwelveEarthlyBranches,
    hi_ten: TenCelestialStems,
    hi_twelve: TwelveEarthlyBranches,

    kubou: (TwelveEarthlyBranches, TwelveEarthlyBranches),

    func_selection: usize,
    counter: i32,
    is_win_large: bool,
    checked: bool,
    radio_selection1: u8,
    radio_selection2: u8,
    selectable1: u8,
    input_text_single: String,
    input_text_multi: String,
    slider_value: f32,
    slider_value_int: u32,
    drag_value_f: f32,
    drag_value_i: i32,
    progress: f32,
    combobox_selection: usize,
    show_details: bool,
    grid_name: String,
    grid_age: u32,
    grid_job: String,
    label_checked: bool,
}

// アプリの初期値
impl Default for DivinationApp<'_> {
    fn default() -> Self {
        Self {
            dialogs: Dialogs::default(),

            use_now: true,
            detail: true,
            visual_mode: VisualMode::Light,
            yyyy_mm_dd: String::new(),
            hh_mm_ss: String::new(),
            etoreki: String::new(),

            nen_ten: TenCelestialStems::None,
            nen_twelve: TwelveEarthlyBranches::None,
            tsuki_ten: TenCelestialStems::None,
            tsuki_twelve: TwelveEarthlyBranches::None,
            hi_ten: TenCelestialStems::None,
            hi_twelve: TwelveEarthlyBranches::None,

            kubou: (TwelveEarthlyBranches::None, TwelveEarthlyBranches::None),

            func_selection: 0,
            counter: 0,
            is_win_large: false,
            checked: false,
            radio_selection1: 0,
            radio_selection2: 0,
            selectable1: 0,
            input_text_single: String::new(),
            input_text_multi: String::new(),
            slider_value: 0.0,
            slider_value_int: 0,
            drag_value_f: 0.0,
            drag_value_i: 0,
            progress: 0.0,
            combobox_selection: 0,
            show_details: false,
            grid_name: String::new(),
            grid_age: 0,
            grid_job: String::new(),
            label_checked: false,
        }
    }
}

// アプリの表示のコントロール
impl eframe::App for DivinationApp<'_> {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 現在の時刻を使用する場合
        if self.use_now {
            // 0.2秒後に再描画をリクエスト
            ctx.request_repaint_after(Duration::from_millis(200));
        }

        // 表示モードの設定
        match self.visual_mode {
            VisualMode::Dark => {
                // クライアント領域の背景色とテキスト色を設定
                let mut dark_visuals = egui::Visuals::dark();
                dark_visuals.panel_fill = egui::Color32::from_rgb(40, 40, 40);
                dark_visuals.override_text_color =
                    Option::Some(egui::Color32::from_rgb(255, 255, 255));
                dark_visuals.text_alpha_from_coverage =
                    egui::epaint::AlphaFromCoverage::DARK_MODE_DEFAULT;
                ctx.set_visuals(dark_visuals);
            }
            VisualMode::Light => {
                // クライアント領域の背景色とテキスト色を設定
                let mut light_visuals = egui::Visuals::light();
                light_visuals.panel_fill = egui::Color32::from_rgb(243, 243, 243);
                light_visuals.override_text_color = Option::Some(egui::Color32::from_rgb(0, 0, 0));
                light_visuals.text_alpha_from_coverage =
                    egui::epaint::AlphaFromCoverage::Gamma(0.5);
                ctx.set_visuals(light_visuals);
            }
            VisualMode::System => {
                ctx.set_visuals(egui::Visuals::default());
            }
        }

        // メニューバーを表示
        self.show_menubar(ctx); // メニューバー表示

        egui::CentralPanel::default().show(ctx, |ui| {
            // 選択された機能を表示
            let selected_func_item: Option<FuncNames> =
                FromPrimitive::from_usize(self.func_selection);
            if let Some(item) = selected_func_item {
                match item {
                    FuncNames::Application => self.show_appli(ctx, ui), // ボタン
                    FuncNames::Button => self.show_button(ctx, ui),     // ボタン
                    FuncNames::Checbox => self.show_checkbox(ctx, ui),  // チェックボックス
                    FuncNames::Collapse => self.show_collapse(ctx, ui), // 折りたたみ
                    FuncNames::Colmuns => self.show_colmuns(ctx, ui),   // 列分割
                    FuncNames::Combobox => self.show_combobox(ctx, ui), // コンボボックス
                    FuncNames::DragValue => self.show_drag_value(ctx, ui), // 数値入力
                    FuncNames::FirstSample => self.show_first_sample(ctx, ui), // 最初のサンプル
                    FuncNames::Frames => self.show_frames(ctx, ui),     // フレーム
                    FuncNames::Grid => self.show_grid(ctx, ui),         // グリッド
                    FuncNames::HorVer => self.show_hor_ver(ctx, ui),    // 水平・垂直配置
                    FuncNames::HorWrapping => self.show_hor_wrapping(ctx, ui), // 水平ラッピング
                    FuncNames::Hyperlink => self.show_hyperlink(ctx, ui), // ハイパーリンク
                    FuncNames::Image => self.show_image(ctx, ui),       // 画像
                    FuncNames::Layout => self.show_layout(ctx, ui), // レイアウト（左寄せ、右寄せ、中央寄せ）
                    FuncNames::Link => self.show_link(ctx, ui),     // リンク
                    FuncNames::MenuButton => self.show_menu_button(ctx, ui), // メニューバー
                    FuncNames::Panels => self.show_panels(ctx, ui), //クライアント領域のパネル分割
                    FuncNames::ProgressBar => self.show_progress_bar(ctx, ui), // プログレスバー
                    FuncNames::RadioButton => self.show_radio_button(ctx, ui), // ラジオボタン
                    FuncNames::Scroll => self.show_scroll(ctx, ui), // スクロール
                    FuncNames::Selectables => self.show_selectables(ctx, ui), // 選択ラベル、選択値
                    FuncNames::Slider => self.show_slider(ctx, ui), // スライダー
                    FuncNames::SpaceAndSeparator => self.show_space_and_separator(ctx, ui), //余白・区切り・インデント
                    FuncNames::Spinner => self.show_spinner(ctx, ui), // スピナー
                    FuncNames::TextInput => self.show_text_input(ctx, ui), // テキスト入力
                    FuncNames::Texts => self.show_texts(ctx, ui),     // テキスト表示
                }
            }
        });

        // ダイアログ表示のための処理
        self.dialogs.show(ctx);
    }
}

/// 日時情報に関するのテスト
fn date_time_test() {
    /*
    let utc_datetime: DateTime<Utc> = Utc::now();
    let utc_date: Date<Utc> = Utc::today();

    println!("{}", utc_datetime);
    println!("{}", utc_date);
    */
    //let local_date: Date<Local> = Local::today();
    //println!("{}", local_date);

    let now: DateTime<Local> = Local::now();

    // ミリ秒を取得（秒未満のナノ秒を100万で割る）
    let millis = now.timestamp_subsec_millis();

    println!("{now}");
    println!(
        "{:04}年{:02}月{:02}日 {:02}時{:02}分{:02}.{:03}秒",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        millis,
    );
}

/// 使用するフォントを設定
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 日本語フォントを追加
    /*
    fonts.font_data.insert(
        "NotoSansCJKjp-Regular".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansCJKjp-Regular.otf"
        ))),
    );
    */
    fonts.font_data.insert(
        "NotoSansCJKjp-VF".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansCJKjp-VF.otf"
        ))),
    );
    fonts.font_data.insert(
        "NotoSansMonoCJKjp-VF".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansMonoCJKjp-VF.otf"
        ))),
    );

    // Proportional: UIテキスト用
    if let Some(proportional_font) = fonts.families.get_mut(&FontFamily::Proportional) {
        proportional_font.insert(0, "NotoSansCJKjp-VF".to_owned());
    } else {
        println!("proportional_font error.");
    }

    // Monospace: 等福フォント用
    if let Some(monospace_font) = fonts.families.get_mut(&FontFamily::Monospace) {
        monospace_font.insert(0, "NotoSansMonoCJKjp-VF".to_owned());
    } else {
        println!("monospace_font error.");
    }
    /*
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "NotoSansMonoCJKjp-VF".to_owned());
        */

    ctx.set_fonts(fonts);
}

//////////////////////
// アプリの実装
impl DivinationApp<'_> {
    /// アプリのコンストラクター
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 日時のテスト
        date_time_test();

        // フォントを設定
        setup_fonts(&cc.egui_ctx);

        // イメージローダーをインストール
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // デフォルトのオブジェクトを作成
        Default::default()
    }

    /// アプリの画面表示
    fn show_appli(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ////////////////////////////////////////////////////////////////////////////
        ui.add_space(10.0);

        GuiTools::show_titled_frame(ctx, ui, "卦", |ui| {
            ui.vertical(|ui| {
                // 現在の時間表示部分
                ui.horizontal(|ui| {
                    if self.use_now {
                        // 現在の日時情報を更新
                        let now: DateTime<Local> = Local::now();
                        self.yyyy_mm_dd =
                            format!(" {:0>4}/{:0>2}/{:0>2}", now.year(), now.month(), now.day());
                        self.hh_mm_ss = format!(
                            " {:0>2}:{:0>2}:{:0>2}",
                            now.hour(),
                            now.minute(),
                            now.second()
                        );
                    }
                    ui.vertical(|ui| {
                        ui.label(" ");
                        let res = ui.checkbox(&mut self.use_now, "現在 → ");
                        if (res).changed() {
                            if self.use_now {
                                // 再描画をリクエスト
                                ctx.request_repaint();
                            }
                            println!("チェックボックスの状態が変わりました: {}", self.use_now);
                        }
                        ui.label("占った日");
                    });
                    ui.vertical(|ui| {
                        let mut width = 0.0;
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                let label_text = "(yyyy/mm/dd)";
                                let size = GuiTools::get_display_size(ctx, ui, label_text);
                                //ui.label(egui::RichText::new(label_text).monospace());
                                ui.label(label_text);
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.yyyy_mm_dd)
                                        .desired_width(size.x),
                                );
                                width += size.x;
                            });
                            ui.vertical(|ui| {
                                let label_text = "(hh:mm:ss)";
                                let size = GuiTools::get_display_size(ctx, ui, label_text);
                                ui.label(label_text);
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.hh_mm_ss)
                                        .desired_width(size.x),
                                );
                                width += size.x;
                            });
                            ui.vertical(|ui| {
                                let label_text = "干支歴";
                                let size =
                                    GuiTools::get_display_size(ctx, ui, "甲子年甲子月甲子日");
                                ui.add_sized([size.x, size.y], egui::Label::new(label_text)); // 幅を指定して中央に表示
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.etoreki)
                                        .desired_width(size.x),
                                );
                                width += size.x;
                            });
                        });
                        ui.horizontal(|ui| {
                            ui.add_space(width / 3.0);
                            if ui.button("↓").clicked() {
                                match TextTools::parse_date(&self.yyyy_mm_dd) {
                                    Some((yyyy, mm, dd)) => {
                                        self.dialogs.info(
                                            "テスト",
                                            format!("{yyyy}/{mm}/{dd}が指定されました。"),
                                        );
                                    }
                                    None => {
                                        self.dialogs.error(
                                            "エラー",
                                            format!(
                                                "日付の指定(yyyy/mm/dd)が正しくありません。\n[{}]",
                                                self.yyyy_mm_dd
                                            ),
                                        );
                                    }
                                }
                                match TextTools::parse_time(&self.hh_mm_ss) {
                                    Some((hh, mm, ss)) => {
                                        self.dialogs.info(
                                            "テスト",
                                            format!("{hh}:{mm}:{ss}が指定されました。"),
                                        );
                                    }
                                    None => {
                                        self.dialogs.error(
                                            "エラー",
                                            format!(
                                                "時刻の指定(hh:mm:ss)が正しくありません。\n[{}]",
                                                self.hh_mm_ss
                                            ),
                                        );
                                    }
                                }
                            }
                            ui.add_space(width / 3.0);
                            ui.label("↓");
                        });
                    });
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("計算").clicked() {
                                println!("計算ボタンが押されました。");
                            }
                            if ui.button("クリア").clicked() {
                                println!("クリアボタンが押されました。");
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("コピー").clicked() {
                                println!("コピーボタンが押されました。");
                            }
                            let res = ui.checkbox(&mut self.detail, "詳細");
                            if (res).changed() {
                                println!(
                                    "詳細チェックボックスの状態が変わりました: {}",
                                    self.detail
                                );
                            }
                        });
                    });
                    /*
                    ui.vertical(|ui| {
                        ui.label("(hh:mm:ss)");
                        ui.text_edit_singleline(&mut self.input_text_single);
                        ui.label("　");
                    });
                    ui.vertical(|ui| {
                        ui.label("干支歴");
                        ui.text_edit_singleline(&mut self.input_text_single);
                        ui.label("　");
                    });
                    let _btn = ui.button("ボタン1");
                    let _btn = ui.button("ボタン2");
                    let _btn = ui.button("ボタン3");
                    */
                });

                // 十干十二支での日付表示部分
                ui.horizontal(|ui| {
                    let size = GuiTools::get_display_size(ctx, ui, "甲");

                    egui::ComboBox::from_id_salt("NEN_TEN")
                        .selected_text(self.nen_ten.as_string())
                        .width(size.x * 3.5)
                        .show_ui(ui, |ui| {
                            for item in TenCelestialStems::iter() {
                                ui.selectable_value(&mut self.nen_ten, item, item.as_string());
                            }
                        });
                    egui::ComboBox::from_id_salt("NEN_TWELVE")
                        .selected_text(self.nen_twelve.as_string())
                        .width(size.x * 3.5)
                        .show_ui(ui, |ui| {
                            for item in TwelveEarthlyBranches::iter() {
                                ui.selectable_value(&mut self.nen_twelve, item, item.as_string());
                            }
                        });
                    ui.label("年");

                    egui::ComboBox::from_id_salt("TSUKI_TEN")
                        .selected_text(self.tsuki_ten.as_string())
                        .width(size.x * 3.5)
                        .show_ui(ui, |ui| {
                            for item in TenCelestialStems::iter() {
                                ui.selectable_value(&mut self.tsuki_ten, item, item.as_string());
                            }
                        });
                    egui::ComboBox::from_id_salt("TSUKI_TWELVE")
                        .selected_text(self.tsuki_twelve.as_string())
                        .width(size.x * 3.5)
                        .show_ui(ui, |ui| {
                            for item in TwelveEarthlyBranches::iter() {
                                ui.selectable_value(&mut self.tsuki_twelve, item, item.as_string());
                            }
                        });
                    ui.label("月");

                    egui::ComboBox::from_id_salt("HI_TEN")
                        .selected_text(self.hi_ten.as_string())
                        .width(size.x * 3.5)
                        .show_ui(ui, |ui| {
                            for item in TenCelestialStems::iter() {
                                ui.selectable_value(&mut self.hi_ten, item, item.as_string());
                            }
                        });
                    egui::ComboBox::from_id_salt("HI_TWELVE")
                        .selected_text(self.hi_twelve.as_string())
                        .width(size.x * 3.5)
                        .show_ui(ui, |ui| {
                            for item in TwelveEarthlyBranches::iter() {
                                ui.selectable_value(&mut self.hi_twelve, item, item.as_string());
                            }
                        });
                    ui.label("日");

                    ui.add_space(size.x);
                    ui.label("空亡 - ");
                    ui.label(format!(
                        "{}, {}",
                        self.kubou.0.as_string(),
                        self.kubou.1.as_string()
                    ));
                });
            });
        });
    }

    /// 機能選択用コンボボックスのサンプル
    fn show_select_func(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let selected_func_item: Option<FuncNames> = FromPrimitive::from_usize(self.func_selection);
        let select_text = if let Some(text) = selected_func_item {
            text.to_string()
        } else {
            "<Unknown>".to_owned()
        };

        egui::ComboBox::from_id_salt("Difficulty")
            .selected_text(select_text)
            .show_ui(ui, |ui| {
                for (i, item) in FuncNames::iter().enumerate() {
                    ui.selectable_value(&mut self.func_selection, i, item.to_string());
                }
            });
    }

    /// メニューバー表示のサンプル
    fn show_menubar(&mut self, _ctx: &egui::Context) {
        // The top panel is often a good place for a menu bar:
        egui::TopBottomPanel::top("top_panel").show(_ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                //egui::widgets::global_theme_preference_buttons(ui);

                // メニュー
                if ui.button("Light Mode").clicked() {
                    self.visual_mode = VisualMode::Light;
                }
                if ui.button("Dark Mode").clicked() {
                    self.visual_mode = VisualMode::Dark;
                }
                if ui.button("System Mode").clicked() {
                    self.visual_mode = VisualMode::System;
                }

                self.show_select_func(_ctx, ui);
            });
        });
    }

    /// 最初のサンプル
    fn show_first_sample(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("egui first app");

        if ui.button("カウント +1").clicked() {
            self.counter += 1;
        }

        ui.label(format!("counter = {}", self.counter));
    }

    /// ボタン表示のサンプル
    fn show_button(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.button("Single Click").clicked() {
            self.counter += 1;
        }
        if ui.button("Double Click").double_clicked() {
            self.counter += 2;
        }
        ui.label(format!("counter = {}", self.counter));
    }

    /// チェックボックスのサンプル
    fn show_checkbox(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let res = ui.checkbox(&mut self.checked, "チェックボックス");
        ui.label(if self.checked {
            "チェックしています"
        } else {
            "チェックしていません"
        });
        if (res).changed() {
            println!("チェックボックスの状態が変わりました: {}", self.checked);
        }
    }

    /// ラジオボタンのサンプル
    fn show_radio_button(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading(format!("難易度 : {}", self.radio_selection1));
        ui.radio_value(&mut self.radio_selection1, 0, "Easy");
        ui.radio_value(&mut self.radio_selection1, 1, "Normal");
        ui.radio_value(&mut self.radio_selection1, 2, "Hard");

        ui.heading(format!("種族 : {}", self.radio_selection2));
        ui.radio_value(&mut self.radio_selection2, 0, "Human");
        ui.radio_value(&mut self.radio_selection2, 1, "Goblin");
        ui.radio_value(&mut self.radio_selection2, 2, "Fairy");
    }

    /// 選択ラベル、選択値のサンプル
    fn show_selectables(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading(format!("難易度 : {}", self.selectable1));
        ui.selectable_value(&mut self.selectable1, 0, "Easy");
        ui.selectable_value(&mut self.selectable1, 1, "Normal");
        ui.selectable_value(&mut self.selectable1, 2, "Hard");

        ui.heading(format!("選択ラベル : {}", self.checked));
        let res = ui.selectable_label(self.checked, "チェックして下さい");
        if res.clicked() {
            self.checked = !self.checked;
        }
    }

    /// テキスト入力のサンプル
    fn show_text_input(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.input_text_single);
        ui.text_edit_multiline(&mut self.input_text_multi);
    }

    /// テキスト表示のサンプル
    fn show_texts(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("これは見出し。ailmwlw");
        ui.label("これはラベル。ailmwlw");
        ui.small("これは小さなテキスト。ailmwlw");
        ui.monospace("これは等幅フォント表示。ailmwlw");
        ui.strong("これは太文字表示。ailmwlw");
        ui.colored_label(Color32::RED, "これは赤いテキスト。ailmwlw");

        ui.colored_label(
            Color32::GREEN,
            RichText::new("これは斜体&太字&等幅。aaiillmmwwllww")
                .italics()
                .strong()
                .monospace(),
        );
    }

    /// スライダーのサンプル
    fn show_slider(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.slider_value, 0.0..=1.0).text("小数点スライダー"));
        ui.add(egui::Slider::new(&mut self.slider_value_int, 0..=10).text("整数スライダー"));
    }

    /// 数値入力のサンプル
    fn show_drag_value(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add(
            DragValue::new(&mut self.drag_value_f)
                .speed(0.1)
                .prefix("値: "),
        );
        ui.add(
            DragValue::new(&mut self.drag_value_i)
                .speed(1)
                .prefix("整数値: "),
        );
    }

    /// プログレスバーのサンプル
    fn show_progress_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        self.progress += 0.01;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }

        ui.add(
            egui::ProgressBar::new(self.progress)
                .text(format!("ロード中...{:.1}%", self.progress * 100.0)),
        );
    }

    /// メニューバーのサンプル
    fn show_menu_button(&self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("my_top_panel").show(ctx, |ui| {
            egui::containers::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("ファイル", |ui| {
                    if ui.button("新規作成").clicked() {
                        println!("新規作成がクリックされました");
                        ui.close();
                    }
                    if ui.button("開く...").clicked() {
                        println!("開く...がクリックされました");
                        ui.close();
                    }
                    if ui.button("保存").clicked() {
                        println!("保存がクリックされました");
                        ui.close();
                    }
                    ui.menu_button("その他", |ui| {
                        if ui.button("その他1").clicked() {
                            println!("その他1がクリックされました");
                            ui.close();
                        }
                        if ui.button("その他2").clicked() {
                            println!("その他2がクリックされました");
                            ui.close();
                        }
                    });
                });
                ui.menu_button("編集", |ui| {
                    if ui.button("元に戻す").clicked() {
                        println!("元に戻すがクリックされました");
                        ui.close();
                    }
                    if ui.button("やり直し").clicked() {
                        println!("やり直しがクリックされました");
                        ui.close();
                    }
                });
            });
        });
    }

    /// コンボボックスのサンプル
    fn show_combobox(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let items = ["Easy", "Normal", "Hard"];
        let selected_text = if let Some(select_text) = items.get(self.combobox_selection) {
            (*select_text).to_owned()
        } else {
            "-----".to_owned()
        };
        ui.label(format!("[{selected_text}]"));
        //ui.label(format!("[{}]", items.get(self.combobox_selection)));
        egui::ComboBox::from_id_salt("Difficulty")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for (i, item) in items.iter().enumerate() {
                    ui.selectable_value(&mut self.combobox_selection, i, (*item).to_owned());
                }
            });
    }

    /// スピナーのサンプル
    fn show_spinner(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.spinner();
    }

    /// 画像表示のサンプル
    fn show_image(&self, _ctx: &egui::Context, ui: &egui::Ui) {
        let egui_icon = egui::include_image!("../assets/images/splash.png");
        let image = egui::Image::new(egui_icon);
        image.paint_at(
            ui,
            Rect::from_two_pos(Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 300.0, y: 300.0 }),
        );
    }

    /// ハイパーリンクのサンプル
    fn show_hyperlink(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.hyperlink("https://www.rust-lang.org/");
        ui.hyperlink_to("Rust公式サイト", "https://www.rust-lang.org/");
    }

    /// リンクのサンプル
    fn show_link(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.link("能力詳細表示").clicked() {
            self.show_details = !self.show_details;
        }
        if self.show_details {
            ui.label("  HP: 100");
            ui.label("  MP: 60");
            ui.label("  AP: 90");
        }
    }

    /// フレームのサンプル
    fn show_frames(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ////////////////////////////////////////////////////////////////////////////
        ui.add_space(30.0);

        GuiTools::show_titled_frame(ctx, ui, "タイトル111111", |ui| {
            ui.add_space(5.0);
            ui.label("グループフレーム     テスト");
            ui.label("グループフレーム     テスト");
            ui.label("グループフレーム     テスト");
        });

        ////////////////////////////////////////////////////////////////////////////

        ui.add_space(30.0);
        egui::Frame::menu(ui.style()).show(ui, |ui| {
            ui.label("メニューフレーム");
        });
        ui.add_space(30.0);
        egui::Frame::window(ui.style()).show(ui, |ui| {
            ui.label("ウィンドウフレーム");
        });
        ui.add_space(30.0);
        egui::Frame::popup(ui.style()).show(ui, |ui| {
            ui.label("ポップアップフレーム");
        });
        ui.add_space(30.0);
        egui::Frame::central_panel(ui.style()).show(ui, |ui| {
            ui.label("セントラルパネルフレーム");
        });
        ui.add_space(30.0);
        egui::Frame::new().show(ui, |ui| {
            ui.label("フレームなし");
        });
    }

    /// クライアント領域のパネル分割のサンプル
    fn show_panels(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("top_panel")
            //.resizable(false)
            .show(ctx, |ui| {
                //self.show_frames(ctx, ui);
                ui.horizontal(|ui| {
                    ui.label("トップパネル");

                    let _btn = ui.button("ボタン1");
                    let _btn = ui.button("ボタン2");
                    let _btn = ui.button("ボタン3");

                    let button1 = ui.button("サイズ変更1");
                    if self.is_win_large {
                        button1.request_focus();
                    }
                    if button1.clicked() {
                        change_canvas_size(ctx, WIN_SIZE_SMALL.x, WIN_SIZE_SMALL.y);
                        self.is_win_large = false;
                    }

                    let button2 = ui.button("サイズ変更2");
                    if !self.is_win_large {
                        button2.request_focus();
                    }
                    if button2.clicked() {
                        change_canvas_size(ctx, WIN_SIZE_LARGE.x, WIN_SIZE_LARGE.y);
                        self.is_win_large = true;
                    }
                });
            });
        egui::TopBottomPanel::bottom("bottom_panel")
            //.resizable(false)
            .show(ctx, |ui| {
                ui.label("ボトムパネル");
                //self.show_frames(ctx, ui);
                ui.horizontal(|ui| {
                    let _btn = ui.button("ボタン1");
                    let _btn = ui.button("ボタン2");
                    let _btn = ui.button("ボタン3");
                });
            });
        egui::SidePanel::left("left_panel")
            .resizable(true)
            //.default_width(100.0)
            .show(ctx, |ui| {
                ui.label("レフトパネル");
                self.show_frames(ctx, ui);
            });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            //.default_width(100.0)
            .show(ctx, |ui| {
                ui.label("ライトパネル");
                self.show_frames(ctx, ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("セントラルパネル");
            //self.show_frames(ctx, ui);
            ui.horizontal(|ui| {
                let _btn = ui.button("ボタン1");
                let _btn = ui.button("ボタン2");
                let _btn = ui.button("ボタン3");
            });
            ui.vertical(|ui| {
                let _btn = ui.button("ボタンA");
                let _btn = ui.button("ボタンB");
                let _btn = ui.button("ボタンC");
            });
        });
    }

    /// 水平・垂直配置のサンプル
    fn show_hor_ver(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let _btn = ui.button("ボタン1");
            let _btn = ui.button("ボタン2");
            let _btn = ui.button("ボタン3");
        });
        ui.vertical(|ui| {
            let _btn = ui.button("ボタンA");
            let _btn = ui.button("ボタンB");
            let _btn = ui.button("ボタンC");
        });
    }

    /// 水平ラッピングのサンプル
    fn show_hor_wrapping(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for i in 1..=20 {
                let _btn = ui.button(format!("ボタン {i}"));
            }
        });
    }

    /// 列分割のサンプル
    fn show_colmuns(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.columns(3, |columns| {
            if let Some(columns_ui) = columns.get_mut(0) {
                columns_ui.label("カラム 1");
            }
            if let Some(columns_ui) = columns.get_mut(1) {
                let _btn = columns_ui.button("カラム 2");
            }
            if let Some(columns_ui) = columns.get_mut(2) {
                let select_label = columns_ui.selectable_label(self.label_checked, "カラム 3");
                if select_label.clicked() {
                    self.label_checked ^= true;
                }
            }
        });
    }

    /// グリッドのサンプル
    fn show_grid(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Grid::new("setting_grid").show(ui, |ui| {
            ui.label("1");
            ui.label("12345");
            ui.label("1234567890");
            ui.label("12345");
            ui.label("abcdef");
            ui.end_row();

            ui.label("20");
            ui.label("123");
            ui.label("123456");
            ui.label("1234567890");
            ui.end_row();

            ui.label("30");
            ui.label("123");
            ui.label("123456");
            ui.label("1234567890");
            ui.label("abcdefghijk");
            ui.end_row();

            ui.label("名前:");
            ui.add_sized(
                egui::vec2(200.0, 24.0),
                egui::TextEdit::singleline(&mut self.grid_name),
            );
            ui.end_row();

            ui.label("年齢:");
            ui.add(DragValue::new(&mut self.grid_age));
            ui.end_row();

            ui.label("職業:");
            ui.add_sized(
                egui::vec2(200.0, 24.0),
                egui::TextEdit::singleline(&mut self.grid_job),
            );
            ui.end_row();
        });
    }

    /// レイアウト（左寄せ、右寄せ、中央寄せ）のサンプル
    fn show_layout(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.label("左寄せ");
            let _btn = ui.button("左寄せ左寄せ");
        });
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("中央寄せ");
            let _btn = ui.button("中央寄せ中央寄せ");
        });
        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            ui.label("右寄せ");
            let _btn = ui.button("右寄せ右寄せ");
        });
    }

    /// スクロールのサンプル
    fn show_scroll(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, Color32::BLACK))
            .show(ui, |ui| {
                ui.set_max_size(egui::vec2(100.0, 150.0));
                ui.set_min_size(egui::vec2(100.0, 150.0));
                egui::ScrollArea::vertical()
                    .auto_shrink(egui::Vec2b::new(false, false))
                    .show(ui, |ui| {
                        for i in 0..50 {
                            ui.label(format!("Item {i}"));
                        }
                    });
            });
    }

    /// 余白・区切り・インデントのサンプル
    fn show_space_and_separator(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.spacing_mut().indent = 30.0;
        ui.label("Marupeke");
        ui.add_space(10.0);
        ui.separator();

        ui.indent("status", |ui| {
            egui::Grid::new("status_grid").show(ui, |ui| {
                ui.label("職業");
                ui.label("冒険者");
                ui.end_row();
                ui.label("HP");
                ui.label("60 / 100");
                ui.end_row();
                ui.label("MP");
                ui.label("36 / 40");
                ui.end_row();
            })
        });
    }

    /// 折りたたみ表示のサンプル
    fn show_collapse(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("ステータス").show(ui, |ui| {
            ui.label("  HP: 100");
            ui.label("  MP: 60");
            ui.label("  AP: 90");
        });
    }
}
