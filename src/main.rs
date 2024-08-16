use aida64_reader_rs::shm;
use std::{fmt, io::Write};

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    loop {
        stdin.read_line(&mut buf).unwrap();
        buf.clear();

        let t = std::time::SystemTime::now();
        let t_ns = t.duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let t_ns_s = format!("{t_ns}");
        let Ok(data) = shm::read_from_shared_memory() else {
            eprintln!("Read AIDA64 using shm failed, please open AIDA64 > Preferences > hardware monitoring > External Applications > Allow shared memory");
            continue;
        };
        for point in data {
            // https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/
            println!(
                "aida64,tag={tag} {field}={value} {t_ns_s}",
                tag = Escape(point.label),
                field = point.id,
                value = Value(point.value),
            );
            // println!("{:>20} {:>40} {:>20}", point.id, point.label, point.value);
        }
        std::io::stdout().flush().ok();
    }
}

struct Value(String);
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.parse::<i64>().is_ok() || self.0.parse::<f64>().is_ok() {
            f.write_str(&self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

/// escape per https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters
struct Escape(String);
impl fmt::Display for Escape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;
        for ch in self.0.chars() {
            match ch {
                ',' | '=' | ' ' => f.write_char('\\')?,
                _ => {}
            }
            f.write_char(ch)?;
        }
        Ok(())
    }
}
