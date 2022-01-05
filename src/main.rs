use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Commentize")]
struct Cli {
    /// Text file to get comment from
    #[structopt(
        short,
        long,
        parse(from_os_str),
        conflicts_with("comment"),
        required_unless("comment")
    )]
    file: Option<PathBuf>,

    /// Comment to be added
    #[structopt(short, long, conflicts_with("file"), required_unless("file"))]
    comment: Option<String>,

    /// Path to file or directory to commentize in the header
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,

    /// Symbol to commnetize with
    #[structopt(short, long)]
    symbol: Option<String>,

    /// Wall symbol to commnetize with
    #[structopt(short, long)]
    wall: Option<String>,

    /// Height padding length
    #[structopt(short("y"), long("hp"))]
    height_pad: Option<usize>,

    /// Width Padding length
    #[structopt(short("x"), long("wp"))]
    width_pad: Option<usize>,

    /// Left side padding length
    #[structopt(short, long)]
    left: Option<usize>,

    /// Right side padding length
    #[structopt(short, long)]
    right: Option<usize>,

    /// Output the comment
    #[structopt(short, long)]
    output: bool,

    /// Append to file path
    #[structopt(short, long)]
    append: bool,

    /// Modded style
    #[structopt(short, long, conflicts_with("symbol"), conflicts_with("wall"))]
    modded: bool,
}

fn main() {
    let cli = Cli::from_args();

    // return println!("{:?}", cli);

    let mut comment = String::new();
    if let Some(c) = cli.comment {
        comment = c;
    } else if let Some(p) = cli.file {
        if p.is_dir() {
            return println!("The path provided is a directory!");
        }
        match fs::read_to_string(&p) {
            Ok(s) => {
                comment = s;
            }
            Err(why) => return println!("Error! {}", why),
        };
    }

    // This is gonna be the end result
    let mut commentized = String::new();

    // Add a new line first when appending
    if cli.append {
        commentized.push('\n');
    }

    // Symbol to commentize with
    let mut sym = match cli.symbol {
        Some(s) => s,
        None => String::from("*"),
    };

    // Modded option
    let mut modded = String::new();
    if cli.modded {
        sym = String::from("///");
        commentized.push_str("/");
        modded.push_str("");
    } else {
        commentized.push_str("/*");
        modded.push_str(" ");
    }

    // Extract first char from symbol
    let one_sym = sym.chars().collect::<Vec<char>>()[0];

    if let Some(s) = cli.wall {
        sym = String::from(s);
    }

    // Width padding length
    let w_pad_len = match cli.width_pad {
        Some(n) => n,
        None => 2,
    };

    // Height padding length
    let h_pad_len = match cli.height_pad {
        Some(n) => n,
        None => 0,
    };

    // Left padding length
    let left = match cli.left {
        Some(n) => n,
        None => 0,
    };

    // Right padding length
    let right = match cli.right {
        Some(n) => n,
        None => 0,
    };

    // Take the lines of file if put
    let lines = comment
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    // Extract longest line length
    let longest = match lines.iter().max_by(|x, y| x.len().cmp(&y.len())) {
        Some(s) => s.len(),
        None => return println!("Error!: The file provided is empty!"),
    };

    // Result square comment width and height
    let width = longest + (w_pad_len * 2) + left + right + (sym.len() * 2) - 1;
    let height = 2 + lines.len();

    // Spaces for width padding
    let w_pad = (0..w_pad_len).map(|_| " ").collect::<String>();

    // Finished lines for height padding
    let spaces_len = longest + (w_pad_len * 2) + left + right;
    let spaces = (0..spaces_len).map(|_| " ").collect::<String>();
    let h_pad = (0..h_pad_len)
        .map(|_| format!("{}{}{}{}\n", modded, sym, spaces, sym))
        .collect::<String>();

    // Spaces for left and right padding
    let l_pad = (0..left).map(|_| " ").collect::<String>();
    let r_pad = (0..right).map(|_| " ").collect::<String>();

    for h in 0..height {
        if h == 0 {
            let beg = (0..width).map(|_| one_sym).collect::<String>();
            let filling = format!("{}\n{}", beg, h_pad);
            commentized.push_str(&filling);
        } else if h == height - 1 {
            let beg = (0..width).map(|_| one_sym).collect::<String>();
            let end;
            if cli.modded {
                end = "/";
            } else {
                end = "*/";
            }
            let filling = format!("{}{}{}{}\n", h_pad, modded, beg, end);
            commentized.push_str(&filling);
        } else {
            let beg = format!("{}{}{}{}", modded, sym, w_pad, l_pad);
            let end = format!("{}{}{}\n", r_pad, w_pad, sym);
            let len = longest - &lines[h - 1].len();
            let spaces = (0..len).map(|_| " ").collect::<String>();
            let filling = format!("{}{}{}{}", beg, &lines[h - 1], spaces, end);
            commentized.push_str(&filling);
        }
    }

    if cli.output || cli.path == None {
        println!("Comment:\n\n{}", commentized);
    }

    let path = match cli.path {
        Some(p) => p,
        None => return,
    };

    println!("Are you sure you want to commentize the file/files? [yes/no]");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // error
        match input.trim() {
            "yes" => break,
            "no" => {
                return println!("❌ Aborted!");
            }
            _ => continue,
        }
    }

    if !cli.append {
        commentized.push('\n');
    }

    match commentize_file(commentized.as_bytes(), &path, cli.append) {
        Ok(_) => {}
        Err(why) => return println!("Error!: {}", why),
    };

    return println!("✅ Successfully commentized!");
}

// Not effecient because it reads file temporarly in memory
fn prepend_file<P: AsRef<Path>>(data: &[u8], path: &P) -> io::Result<()> {
    let file_string = fs::read_to_string(&path)?;

    let mut data_clone = data.clone();
    let mut data_string = String::new();
    data_clone.read_to_string(&mut data_string)?;
    if file_string.starts_with(&data_string) {
        return Ok(());
    }

    fs::write(&path, &data)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path)
        .unwrap();
    file.write_all(file_string.as_bytes())?;
    Ok(())
}

fn append_file<P: AsRef<Path>>(data: &[u8], path: &P) -> io::Result<()> {
    let file_string = fs::read_to_string(&path)?;
    let mut data_clone = data.clone();
    let mut data_string = String::new();
    data_clone.read_to_string(&mut data_string)?;
    if file_string.ends_with(&data_string) {
        return Ok(());
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path)?;
    file.write_all(&data)?;
    Ok(())
}

fn commentize_file<P>(data: &[u8], path: &P, append: bool) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if path.as_ref().is_dir() {
        for entry in path.as_ref().read_dir()? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                commentize_file(&data, &p, append)?;
            } else {
                if append {
                    append_file(&data, &p)?;
                } else {
                    prepend_file(&data, &p)?;
                }
            }
        }
    } else {
        if append {
            append_file(&data, &path)?;
        } else {
            prepend_file(&data, &path)?;
        }
    }

    Ok(())
}
