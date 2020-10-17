pub type MetricType = &'static str;

#[allow(dead_code)]
pub(crate) const METRIC_COUNTER: MetricType = "counter";
pub(crate) const METRIC_GAUGE: MetricType = "gauge";
#[allow(dead_code)]
pub(crate) const METRIC_HISTOGRAM: MetricType = "histogram";
#[allow(dead_code)]
pub(crate) const METRIC_SUMMARY: MetricType = "summary";

pub enum Metric {
    Float { value: f64, metric_type: MetricType, help: Box<str> }
}

impl Metric {
    pub fn format(&self, key: &str) -> String {
        let mut formatted = String::new();

        match self {
            Metric::Float { value, metric_type, help } => {
                formatted.push_str(&format!("# HELP {} {}", key, help)[..]);
                formatted.push_str("\n");

                formatted.push_str(&format!("# TYPE {} {}", key, metric_type)[..]);
                formatted.push_str("\n");

                formatted.push_str(&format!("{}{{}} {}", key, value)[..]);
                formatted.push_str("\n");
            }
        }

        formatted
    }
}
