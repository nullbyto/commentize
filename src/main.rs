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

    /// Title to add on top of the box
    #[structopt(short, long)]
    title: Option<String>,

    /// Path to file or directory to commentize in header/footer
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,

    /// Symbol to commentize with
    #[structopt(short, long)]
    symbol: Option<String>,

    /// Wall symbol to commentize with
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

    /// Move box from beginning of line
    #[structopt(short, long("move"), requires("box-only"))]
    mv: Option<usize>,

    /// Output result when commenting files
    #[structopt(short, long, requires("path"))]
    output: bool,

    /// Append to file path
    #[structopt(short, long)]
    append: bool,

    /// Modded style
    #[structopt(long, conflicts_with("symbol"), conflicts_with("wall"), conflicts_with("box_only"))]
    modded: bool,

    /// Comment box only (without comment delimiter)
    #[structopt(short, long, conflicts_with("path"))]
    box_only: bool,
}

fn main() {
    let cli = Cli::from_args();

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

    // Title to add on top of the box
    let title = match cli.title {
        Some(s) => s,
        None => String::from(""),
    };

    let title_len = title.chars().count();

    // Extract first char from symbol
    let one_sym = sym.chars().collect::<Vec<char>>()[0];

    if let Some(s) = cli.wall {
        sym = String::from(s);
    }

    let delimiter;

    // Modded option
    let mut modded = String::new();
    if cli.modded {
        sym = String::from("///");
        // commentized.push_str("/");
        delimiter = "/".to_string();
        modded.push_str("");
    } else if cli.box_only {
        // commentized.push_str(&sym);
        delimiter = one_sym.to_string();
    } else {
        // commentized.push_str("/*");
        delimiter = "/*".to_string();
        modded.push_str(" ");
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

    // Move box
    let mv = match cli.mv {
        Some(n) => n,
        None => 0,
    };

    // Take the lines of file if put
    let lines = comment
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    // Extract longest line length
    // chars().count() is used for chars that take more than 1 byte
    let mut longest = match lines.iter().max_by(|x, y| x.chars().count().cmp(&y.chars().count())) {
        Some(s) => s.chars().count(),
        None => return println!("Error!: The file provided is empty!"),
    };

    if title_len > longest {
        longest = title_len;
    }

    // Result square comment width and height
    let width = longest + (w_pad_len * 2) + left + right + (sym.len() * 2) - 1;
    let height = 2 + lines.len();

    // Spaces for width padding
    let w_pad = (0..w_pad_len).map(|_| " ").collect::<String>();

    // Spaces for box moving
    let mv_pad = (0..mv).map(|_| " ").collect::<String>();

    // Finished lines for height padding
    let spaces_len = longest + (w_pad_len * 2) + left + right;
    let spaces = (0..spaces_len).map(|_| " ").collect::<String>();
    let h_pad = (0..h_pad_len)
        .map(|_| format!("{}{}{}{}{}\n", mv_pad, modded, sym, spaces, sym))
        .collect::<String>();

    // Spaces for left and right padding
    let l_pad = (0..left).map(|_| " ").collect::<String>();
    let r_pad = (0..right).map(|_| " ").collect::<String>();


    //////////////////////////////////////////////////////////////////////////
    // Title
    //////////////////////////////////////////////////////////////////////////
    let mut top = String::from("");
    if title_len != 0 {
        let beg = (0..width+1).map(|_| one_sym).collect::<String>();
        let spaces_len = (width+1)-(title_len)-2*(sym.len());
        let spaces = (0..(spaces_len/2)).map(|_| " ").collect::<String>();
        if spaces_len % 2 == 0 {
            top = format!("{}{}\n{}{}{}{}{}{}", mv_pad, beg, mv_pad, sym, spaces, title, spaces, sym);
        } else {
            top = format!("{}{}\n{}{}{}{}{} {}", mv_pad, beg, mv_pad, sym, spaces, title, spaces, sym);
        }
    }

    for h in 0..height {
        if h == 0 {
            let beg = (0..width).map(|_| one_sym).collect::<String>();
            let filling = format!("{}\n{}{}{}\n{}", top, mv_pad, delimiter, beg, h_pad);
            commentized.push_str(&filling);
        } else if h == height - 1 {
            let beg = (0..width).map(|_| one_sym).collect::<String>();
            let end;
            if cli.modded {
                end = "/".to_string();
            } else if cli.box_only {
                end = one_sym.to_string();
            } else {
                end = "*/".to_string();
            }
            let filling = format!("{}{}{}{}{}\n", h_pad, modded, mv_pad, beg, end);
            commentized.push_str(&filling);
        } else {
            let beg = format!("{}{}{}{}{}", mv_pad, modded, sym, w_pad, l_pad);
            let end = format!("{}{}{}\n", r_pad, w_pad, sym);
            let len = longest - &lines[h - 1].chars().count(); // chars().count() for 2 bytes char
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
