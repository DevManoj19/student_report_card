use std::io;
use std::fs::File;
use std::io::{BufWriter};
use chrono::Local;
use printpdf::*;

fn calculate_average(total: f32, count: usize) -> f32 {
    total / count as f32
}

fn assign_grade(avg: f32) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn main() {
    let mut input = String::new();

    // Student Info
    println!("Enter Student Name:");
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();
    input.clear();

    println!("Enter School Grade:");
    io::stdin().read_line(&mut input).unwrap();
    let year = input.trim().to_string();
    input.clear();

    println!("Enter Term:");
    io::stdin().read_line(&mut input).unwrap();
    let term = input.trim().to_string();
    input.clear();

    println!("Enter Teacher Name:");
    io::stdin().read_line(&mut input).unwrap();
    let teacher = input.trim().to_string();
    input.clear();

    let date = Local::now().format("%d-%m-%Y").to_string();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    let mut subjects = Vec::new();
    let mut total_marks = 0.0;

    for i in 1..=n {
        println!("Enter name of subject {}:", i);
        io::stdin().read_line(&mut input).unwrap();
        let subject = input.trim().to_string();
        input.clear();

        println!("Enter marks for {}:", subject);
        io::stdin().read_line(&mut input).unwrap();
        let mark: f32 = input.trim().parse().expect("Enter a valid number");
        input.clear();

        total_marks += mark;
        subjects.push((subject, mark));
    }

    let average = calculate_average(total_marks, n);
    let grade = assign_grade(average);

    // ---------------------
    // Console Report Output
    // ---------------------
    println!("\n===============================");
    println!("        STUDENT REPORT CARD    ");
    println!("===============================");
    println!("Name     : {}", name);
    println!("Year     : {}", year);
    println!("Term     : {}", term);
    println!("Teacher  : {}", teacher);
    println!("Date     : {}", date);
    println!("-------------------------------");
    println!("{:<20} {:<10}", "Subject", "Marks");
    println!("-------------------------------");

    for (subject, mark) in &subjects {
        println!("{:<20} {:<10.2}", subject, mark);
    }

    println!("-------------------------------");
    println!("{:<20} {:<10.2}", "Total Marks", total_marks);
    println!("{:<20} {:<10.2}", "Average", average);
    println!("{:<20} {}", "Final Grade", grade);
    println!("===============================\n");

    // ---------------------
   // ---------------------
// ---------------------
    // PDF Generation Logic
    // ---------------------
    let (doc, page1, layer1) = PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Change the font path to a valid one on your system (Linux/Windows)
    let font = doc.add_external_font(
        File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf").unwrap()
    ).unwrap();

    let mut y = 270.0;
    let mut draw_line = |text: &str| {
        current_layer.use_text(text, 12.0, Mm(20.0), Mm(y), &font);

        y -= 10.0;
    };

    draw_line("📘 STUDENT REPORT CARD");
    draw_line(&format!("Name     : {}", name));
    draw_line(&format!("Year     : {}", year));
    draw_line(&format!("Term     : {}", term));
    draw_line(&format!("Teacher  : {}", teacher));
    draw_line(&format!("Date     : {}", date));
    draw_line("-------------------------------------");
    draw_line(&format!("{:<20} {:<10}", "Subject", "Marks"));

    for (subject, mark) in &subjects {
        draw_line(&format!("{:<20} {:<10.2}", subject, mark));
    }

    draw_line("-------------------------------------");
    draw_line(&format!("{:<20} {:<10.2}", "Total Marks", total_marks));
    draw_line(&format!("{:<20} {:<10.2}", "Average", average));
    draw_line(&format!("{:<20} {}", "Final Grade", grade));

    // Save as PDF
    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap())).unwrap();

    println!("✅ PDF generated: report_card.pdf");
}