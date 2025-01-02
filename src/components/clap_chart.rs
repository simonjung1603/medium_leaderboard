use plotly::Plot;
use plotly::layout::*;
use plotly::traces::bar::*;
use plotly::configuration::*;
use plotly::color::Rgb;
use plotly::layout::BarMode;
use dioxus::logger::tracing;
use crate::components::app::SubmissionsByCategory;
use dioxus::prelude::*;
use serde::Serialize;
use crate::models::Submission;
#[cfg(feature = "web")]
use web_sys::js_sys;

#[derive(Serialize)]
struct Chart {
    #[serde(rename = "type")]
    chart_type: &'static str,
    data: ChartData,
    options: ChartOptions,
}

#[derive(Serialize, Default)]
struct ChartData {
    labels: Vec<String>,
    datasets: Vec<Dataset>,
}

#[derive(Serialize)]
struct Dataset {
    label: String,
    data: Vec<DatasetData>,
    #[serde(rename = "backgroundColor")]
    background_colors: Vec<String>,
    #[serde(rename = "borderColor")]
    border_colors: Vec<String>,
    grouped: bool,
}

#[derive(Serialize, Clone)]
struct DatasetData {
    x: String,
    y: i32,
}

impl Dataset {
    fn with_default_colors(label: String, data: Vec<DatasetData>) -> Self {
        /*
        backgroundColor: [
                              'rgba(255, 99, 132, 0.2)',
                              'rgba(255, 159, 64, 0.2)',
                              'rgba(255, 205, 86, 0.2)',
                              'rgba(75, 192, 192, 0.2)',
                              'rgba(54, 162, 235, 0.2)',
                              'rgba(153, 102, 255, 0.2)',
                              'rgba(201, 203, 207, 0.2)'
                            ],
                            borderColor: [
                              'rgb(255, 99, 132)',
                              'rgb(255, 159, 64)',
                              'rgb(255, 205, 86)',
                              'rgb(75, 192, 192)',
                              'rgb(54, 162, 235)',
                              'rgb(153, 102, 255)',
                              'rgb(201, 203, 207)'
                            ],
         */
        Self {
            label,
            data,
            background_colors: vec!["rgba(255, 99, 132, 0.2)".into(), "rgba(255, 159, 64, 0.2)".into(), "rgba(255, 205, 86, 0.2)".into()],
            border_colors: vec!["rgb(255, 99, 132)".into(), "rgb(255, 159, 64)".into(), "rgb(255, 205, 86)".into()],
            grouped: false,
        }
    }
}

#[derive(Serialize, Default)]
struct ChartOptions {
    responsive: bool,
    scales: ScalesOptions,
}

#[derive(Serialize, Default)]
struct ScalesOptions {
    y: Option<ScaleOptions>,
    x: Option<ScaleOptions>,
}

#[derive(Serialize, Default)]
struct ScaleOptions {
    #[serde(rename = "beginAtZero")]
    begin_at_zero: Option<bool>,
    #[serde(rename = "barPercentage")]
    bar_percentage: Option<f32>,
}

/*
{{
                    options: {{
                        plugins: {{
                            legend: {{
                                labels: {{
                                    boxWidth: 0
                                }}
                            }}
                        }},
                        responsive: true,
                        scales: {{
                            y: {{ beginAtZero: true }}
                        }},
                    }}
                }}
 */

#[component]
pub fn ClapChart(id: String, submissions_by_category: Memo<Option<SubmissionsByCategory>>) -> Element {
    let plot_id = id.clone();

    use_effect(move || {
        if let Some(subs) = &*submissions_by_category.read_unchecked() {
            let mut plot = Plot::new();

            plot.add_trace(Bar::new(
                subs.poetry.iter().map(|sub| sub.title.clone()).collect(),
                subs.poetry.iter().map(|sub| sub.clap_count).collect(),
            ).name("Poetry").clip_on_axis(false));
            plot.add_trace(Bar::new(
                subs.fiction.iter().map(|sub| sub.title.clone()).collect(),
                subs.fiction.iter().map(|sub| sub.clap_count).collect(),
            ).name("Fiction").clip_on_axis(false));
            plot.add_trace(Bar::new(
                subs.essay.iter().map(|sub| sub.title.clone()).collect(),
                subs.essay.iter().map(|sub| sub.clap_count).collect(),
            ).name("Personal Essay").clip_on_axis(false));

            plot.set_layout(Layout::new()
                .bar_mode(BarMode::Group)
                .paper_background_color(Rgb::new(20, 22, 26))
                .plot_background_color(Rgb::new(20, 22, 26))
                .margin(Margin::new().bottom(200))
            );

            plot.set_configuration(Configuration::default().display_mode_bar(DisplayModeBar::False));

            let value = plot_id.clone();
            spawn(async move {
                plotly::bindings::new_plot(&value, &plot).await
            });
        }
    });

    rsx! {
        div {class: "box ml-6 mr-6",
            div {
                id: id,
            }
        }
    }
}