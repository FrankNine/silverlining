use fasteval::Error;
use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());

    const COLUMN_COUNT: usize = 10;
    const ROW_COUNT: usize = 8;
    let mut state: [[String; ROW_COUNT]; COLUMN_COUNT] = Default::default();

    system.main_loop(move |_, ui| {
        Window::new("Table")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                let t = ui.begin_table("Table", COLUMN_COUNT);
                for row in 0..ROW_COUNT {
                    ui.table_next_row();
                    for column in 0..COLUMN_COUNT {
                        ui.table_set_column_index(column);
                        if row == 0 && column != 0 {
                            ui.text(format!("{}", column));
                        }

                        if column == 0 && row != 0 {
                            ui.text(format!(
                                "{}",
                                char::from_u32((b'A' + row as u8 - 1) as u32).unwrap()
                            ));
                        }

                        if row == 0 && column == 0 {
                            continue;
                        }

                        if row != 0 && column != 0 {
                            let id = ui.push_id(&format!("r{}c{}", row, column));
                            ui.input_text("", &mut state[column][row]).build();
                            if state[column][row].starts_with("=") {
                                let mut ns = fasteval::EmptyNamespace;
                                let val = fasteval::ez_eval(&state[column][row][1..], &mut ns);
                                state[column][row] = match val {
                                    Ok(f) => f.to_string(),
                                    Err(_) => String::new(),
                                }
                            }
                            id.pop();
                        }
                    }
                }
                t.unwrap().end();
            });
    });
}
