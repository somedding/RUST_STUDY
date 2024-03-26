fn main()
{
    let body = Body::new(163.0, 75.2, "성은");
    body.print_result();
    let body = Body::new(158.0, 55.0, "성은");
    body.print_result();
    let body = Body::new(174.0, 54.2, "성은");
    body.print_result();
}

struct BmiRange
{
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange
{
    fn new(min: f64, max: f64, )
}