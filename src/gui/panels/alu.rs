use crate::utils::{self, convert::from_string_integer};

pub struct AluData {
    // TODO: Is it possible to convert into i16?
    input_a: String,
    input_b: String,
    preset_a: String,
    preset_b: String,
    selector: bool,
    postselector: bool,
    output: String,

    error: String,
}

impl Default for AluData {
    fn default() -> Self {
        Self {
            input_a: "0".to_owned(),
            input_b: "0".to_owned(),
            preset_a: "0".to_owned(),
            preset_b: "0".to_owned(),
            selector: false,
            postselector: false,
            output: "0".to_owned(),
            error: "".to_owned(),
        }
    }
}

pub fn panel_alu(
    // ctx: &mut Context,
    ui: &mut egui::Ui,
    label: &mut String,
    data: &mut AluData,
    frame: &mut eframe::Frame,
) {
    let input_a = &mut data.input_a;
    let input_b = &mut data.input_b;
    let output = &mut data.output;

    ui.label("16-bit ALU");
    ui.horizontal(|ui| {
        ui.label("Input A:");
        ui.add(egui::widgets::TextEdit::singleline(input_a));
    });

    ui.horizontal(|ui| {
        ui.label("Input B:");
        ui.add(egui::widgets::TextEdit::singleline(input_b));
    });

    if ui.button("Run").clicked() {
        let result = from_string_integer(input_a.to_string())
            .and_then(|a| from_string_integer(input_b.to_string()).map(|b| (a, b)));

        match result {
            Ok((a, b)) => {
                let output_b16 = crate::pc::chips::adder::adder_b16(a.as_array_b16, b.as_array_b16);

                let output_i16 = utils::convert::from_b16(output_b16);
                data.output = output_i16.unwrap().to_string(); // TODO: Do we need to check the error?
            }
            Err(e) => {
                data.error = e;
                return;
            }
        }
    }

    ui.horizontal(|ui| {
        ui.label("Result:");
        ui.add(egui::widgets::Label::new(format!("{}", data.output)));
    });

    ui.horizontal(|ui| {
        if data.error != "" {
            ui.label("Error!:");
            ui.add(egui::widgets::Label::new(format!("{}", data.error)));
        }
    });
}
