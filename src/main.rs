use egui::ViewportBuilder;
use calculator::Calculator;

mod calculator;

const BUTTONS: &[&str] = &[
    // Row 1: Clear and parentheses
    "C", "(", ")", "^", "mod",
    // Row 2: Numbers and basic operators
    "7", "8", "9", "/", "*",
    "4", "5", "6", "+", "-",
    "1", "2", "3", ".", "=",
    // Row 3: Zero and constants
    "0", "pi", "e", "abs", "sqrt",
    // Row 4: Trigonometric functions
    "sin", "cos", "tan", "cbrt", "round",
    // Row 5: Logarithmic and rounding functions
    "log", "log10", "floor", "ceil", "="
];

struct CalculatorApp {
    display: String,
    result: String,
    error: Option<String>,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            display: String::new(),
            result: String::new(),
            error: None,
        }
    }
}

impl CalculatorApp {
    fn calculate(&mut self) {
        self.error = None;
        match Calculator::parse(&self.display) {
            Ok(tokens) => {
                let expr = Calculator::expression(tokens);
                match Calculator::evaluate(expr) {
                    Ok(result) => {
                        self.result = format!("{}", result);
                    },
                    Err(e) => {
                        self.error = Some(format!("{:?}", e));
                    }
                }
            },
            Err(e) => {
                self.error = Some(format!("{:?}", e));
            }
        }
    }

    fn handle_input(&mut self, input: &str) {
        match input {
            "C" => {
                self.display.clear();
                self.result.clear();
                self.error = None;
            },
            "=" => self.calculate(),
            _ => {
                self.display.push_str(input);
                // Add opening parenthesis automatically for functions
                match input {
                    "sin" | "cos" | "tan" | "sqrt" | "cbrt" | "log" | "log10" |
                    "abs" | "floor" | "ceil" | "round" => {
                        self.display.push('(');
                    },
                    _ => {}
                }
            }
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Title
                ui.heading("Calculadora Científica Guizar");
                
                // Display area with border and padding
                ui.add_space(10.0);
                egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.monospace(&self.display);
                    });
                    ui.add_space(5.0);
                });
                
                // Result area
                ui.horizontal(|ui| {
                    if let Some(error) = &self.error {
                        ui.colored_label(egui::Color32::RED, error);
                    } else if !self.result.is_empty() {
                        ui.colored_label(egui::Color32::GREEN, format!("= {}", self.result));
                    }
                });

                ui.add_space(20.0);

                // Button grid
                let button_size = egui::vec2(70.0, 40.0);
                let mut col = 0;
                
                ui.horizontal_wrapped(|ui| {
                    for &button in BUTTONS {
                        if col > 0 && col % 5 == 0 {
                            ui.end_row();
                        }
                        
                        let btn = ui.add_sized(
                            button_size,
                            egui::Button::new(button)
                                .fill(match button {
                                    "=" => egui::Color32::from_rgb(0, 150, 0),
                                    "C" => egui::Color32::from_rgb(150, 0, 0),
                                    "sin" | "cos" | "tan" | "log" | "log10" | 
                                    "sqrt" | "cbrt" | "abs" | "floor" | "ceil" | "round" =>
                                        egui::Color32::from_rgb(70, 70, 170),
                                    "pi" | "e" => egui::Color32::from_rgb(170, 70, 70),
                                    "+" | "-" | "*" | "/" | "^" | "mod" =>
                                        egui::Color32::from_rgb(100, 100, 100),
                                    _ => ui.style().visuals.widgets.inactive.bg_fill,
                                })
                        );
                        
                        if btn.clicked() {
                            self.handle_input(button);
                        }
                        
                        col += 1;
                    }
                });

                // Keyboard input handling
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.calculate();
                }
                
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.display.clear();
                    self.result.clear();
                    self.error = None;
                }

                // Help text
                ui.add_space(20.0);
                ui.collapsing("Ayuda", |ui| {
                    ui.label("Atajos de teclado:");
                    ui.label("Enter - Calcular");
                    ui.label("Escape - Borrar");
                    ui.add_space(10.0);
                    ui.label("Uso de las funciones:");
                    ui.label("• Trigonométrico: sin(x), cos(x), tan(x)");
                    ui.label("• Raíces: sqrt(x), cbrt(x)");
                    ui.label("• Logarítmico: log(x), log10(x)");
                    ui.label("• Redondeo: floor(x), ceil(x), round(x)");
                    ui.label("• Otros: abs(x), mod");
                    ui.add_space(10.0);
                    ui.label("Constantes:");
                    ui.label("• pi ≈ 3.14159...");
                    ui.label("• e ≈ 2.71828...");
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 700.0])
            .with_min_inner_size([300.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Calculadora Científica Guizar",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::default())))
    )
}