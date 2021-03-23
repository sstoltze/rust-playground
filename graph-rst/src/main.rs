use std::io::{self, BufRead, Error};
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::symbols;
use tui::text::Span;
use tui::widgets::{Axis, Chart, Dataset, GraphType};
use tui::Terminal;

const CHART_SIZE: f64 = 100.0;

fn prepare_data(points: &Vec<f64>) -> Vec<(f64, f64)> {
    let length = (points.len() - 1) as f64;
    points
        .iter()
        .enumerate()
        .map(|(i, p)| ((i as f64) * (CHART_SIZE / length), *p))
        .collect::<Vec<(f64, f64)>>()
}

fn draw_chart(mut points: Vec<f64>) -> Result<(), Error> {
    let data = prepare_data(&points);
    points.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let (min, max) = (*points.first().unwrap(), *points.last().unwrap());
    let datasets = vec![Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(data.as_slice())];
    let chart = Chart::new(datasets)
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 100.0]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::White))
                .bounds([min, max])
                .labels(
                    [min.to_string(), "0.0".to_string(), max.to_string()]
                        .iter()
                        .cloned()
                        .map(Span::from)
                        .collect(),
                ),
        );
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let original_size = f.size();
        let half_size = Rect {
            height: original_size.height / 2,
            width: original_size.width / 2,
            ..original_size
        };
        f.render_widget(chart, half_size);
    })
}

fn main() -> Result<(), Error> {
    let mut numbers = vec![];
    for line in io::stdin().lock().lines() {
        let f = line.unwrap().trim().parse::<f64>().unwrap();
        numbers.push(f);
    }

    if numbers.is_empty() {
        panic!("No numbers to graph.")
    }
    draw_chart(numbers)
}
