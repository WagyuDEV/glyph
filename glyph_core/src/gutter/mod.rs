pub mod absolute_line_gutter;
pub mod noop_line_gutter;
pub mod relative_line_gutter;

pub trait Gutter: std::fmt::Debug {
    fn get_lines(
        &self,
        total_lines: usize,
        line: usize,
        scroll: usize,
        height: usize,
    ) -> Vec<String>;
}
