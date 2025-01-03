use std::rc::Rc;

use charts_rs::{BarChart, Color, Series, THEME_DARK};
use leptos::{component, view, IntoView};
use num_format::{Buffer, Locale};

use crate::app::{components::DashboardWidget, models::Person};

#[component]
pub fn DashboardChart(persons_data: Vec<Person>) -> impl IntoView {
    let retrieved_persons_data = Rc::new(persons_data.clone());
    let team_count = retrieved_persons_data.len().to_string();
    let mut total_cost = 0;
    let mut latest_member = String::new();
    let mut counter = 0;

    let mut data_vec = Vec::new();
    let mut count_vec: Vec<f32> = Vec::new();

    persons_data.into_iter().for_each(|person| {
        if counter == 0 {
            latest_member = person.name;
        }

        total_cost += person.compensation;

        if !data_vec.contains(&person.title) {
            data_vec.push(person.title);
            count_vec.push(1.0);
        } else {
            let index = data_vec
                .iter()
                .position(|title| title == &person.title)
                .unwrap();
            let num_at_index = count_vec[index];
            count_vec[index] = num_at_index + 1.0;
        }

        counter += 1;
    });

    let mut bar_series = Series::new(String::new(), count_vec);
    bar_series.label_show = true;

    let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
    bar_chart.font_family = String::from("Noto Sans SC");
    bar_chart.background_color = Color::transparent();
    bar_chart.width = 832.0;
    bar_chart.height = 500.0;

    bar_chart.y_axis_hidden = true;

    let mut buf = Buffer::default();
    buf.write_formatted(&total_cost, &Locale::en);
    let total_cost_str = format!("${}", buf.as_str());

    view! {
        <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
            <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
                <DashboardWidget title="Team Members" value=&team_count />
                <DashboardWidget title="Monthly Team Cost" value=&total_cost_str />
                <DashboardWidget title="Just Joined" value=&latest_member />
            </div>

            <div class="max-w-[54rem] mx-auto w-full flex flex-col mt-14 pb-12">
                <div
                    class="w-full max-w-[41rem] h-20 bg-black-200 rounded py-10 px-4 pb-10"
                    inner_html=&bar_chart.svg().unwrap()
                ></div>
            </div>
        </div>
    }
}
