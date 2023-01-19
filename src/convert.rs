
use badgen;
use std::io;
use xml;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

fn attrs_to_badge(v: & Vec<OwnedAttribute>) {
    for a in v {
	if a.name.local_name == "line-rate" {
	    let mut style = badgen::Style::classic();
	    let vstr = a.value.as_str();
	    let value: f32 = vstr.parse().expect("Invalid coverage value");
	    if value < 0.5 {
		style.background = badgen::Color::Red;
	    }
	    else if value < 0.75 {
		style.background = badgen::Color::Yellow;
	    }
	    else {
		style.background = badgen::Color::Green;
	    }
	    let vperc = format!("{:.1} %", value * 100.0);
	    let cov = badgen::badge(&style, vperc.as_str(), Some("Coverage")).unwrap();
	    println!("{}", cov);
	    return;
	}
    }
    panic!("NO COVERAGE VALUES FOUND")
}

pub fn convert() {
    let parser = EventReader::new(io::stdin());
    for ev in parser {
	match ev {
	    Ok(XmlEvent::StartElement { name, attributes, .. }) => {
		if name.local_name == "coverage" {
		    attrs_to_badge(&attributes);
		}
	    }
	    Err(e) => {
		panic!("Could not parse xml! {}", e);
	    }
	    _ => {}
	}
    }
}
