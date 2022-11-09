use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    system.main_loop(move |_, ui| {
        Window::new("Hello world")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text_wrapped("Hello world!");
                ui.text_wrapped("こんにちは世界！");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
        Window::new("Table")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                let t = ui.begin_table("Table", 3);
                for row in 0..4
                {
                    ui.table_next_row();
                    for column in 0..3
                    {
                        ui.table_set_column_index(column);
                        ui.text(format!("Row {} Column {}", row, column));
                    }
                }
                t.unwrap().end();
            });
    });
}
