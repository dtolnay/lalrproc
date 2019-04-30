#[derive(Copy, Clone, Debug)]
pub struct Span(pub proc_macro::Span);

impl Default for Span {
    fn default() -> Self {
        Span(proc_macro::Span::call_site())
    }
}
