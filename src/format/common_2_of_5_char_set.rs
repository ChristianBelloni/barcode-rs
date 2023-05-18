use phf::phf_map;

pub static WIDTH_MAP: phf::Map<char, [Width; 5]> = phf_map! {
    '0' => [Width::Narrow,Width::Narrow,Width::Wide, Width::Wide, Width::Narrow],
    '1' => [Width::Wide,Width::Narrow,Width::Narrow, Width::Narrow, Width::Wide],
    '2' => [Width::Narrow,Width::Wide,Width::Narrow, Width::Narrow, Width::Wide],
    '3' => [Width::Wide,Width::Wide,Width::Narrow, Width::Narrow, Width::Narrow],
    '4' => [Width::Narrow,Width::Narrow,Width::Wide, Width::Narrow, Width::Wide],
    '5' => [Width::Wide,Width::Narrow,Width::Wide, Width::Narrow, Width::Narrow],
    '6' => [Width::Narrow,Width::Wide,Width::Wide, Width::Narrow, Width::Narrow],
    '7' => [Width::Narrow,Width::Narrow,Width::Narrow, Width::Wide, Width::Wide],
    '8' => [Width::Wide,Width::Narrow,Width::Narrow, Width::Wide, Width::Narrow],
    '9' => [Width::Narrow,Width::Wide,Width::Narrow, Width::Wide, Width::Narrow]
};

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Wide,
    Narrow,
}
