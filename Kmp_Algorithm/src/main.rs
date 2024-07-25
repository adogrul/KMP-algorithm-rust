use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

fn get_file_size(path: &str) -> io::Result<u64> {
    let file_metadata = fs::metadata(path)?;
    Ok(file_metadata.len())
}

fn read_all_bytes(path: &str) -> io::Result<Vec<u8>> {
    let start = Instant::now(); // Zaman ölçümüne başla
    
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let duration = start.elapsed(); // Süreyi ölç
    
    println!("Dosya okuma süresi: {:?}", duration); // Süreyi ekrana yazdır
    Ok(buffer)
}

fn compute_lps_array(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}

fn kmp_search(pattern: &[u8], file_path: &str) -> io::Result<()> {
    let file_content = read_all_bytes(file_path)?;
    let n = file_content.len() as usize;
    let m = pattern.len();
    
    if m == 0 || n == 0 {
        eprintln!("Invalid size for pattern or file content");
        return Ok(());
    }

    let lps = compute_lps_array(pattern);

    let mut i = 0; // index for file_content[]
    let mut j = 0; // index for pattern[]

    while i < n {
        if pattern[j] == file_content[i] {
            j += 1;
            i += 1;
        }

        if j == m {
            println!("Found pattern at index {}", i - j);
            j = lps[j - 1];
        } else if i < n && pattern[j] != file_content[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    Ok(())
}

fn sub_dir_list_files(path: &str) -> io::Result<Vec<String>> {
    let mut directories = Vec::new();
    
    // Dizin içeriğini okuma
    let entries = fs::read_dir(path)?;
    let total_entries = entries.count(); // Toplam dosya sayısını bulma
    
    // İlerleme çubuğunu başlat
    let pb = ProgressBar::new(total_entries as u64);
    let style = ProgressStyle::default_bar()
        .template("{bar:40} {percent}% ({elapsed} / {eta})")
        .unwrap_or_else(|e| {
            eprintln!("Error setting progress bar template: {:?}", e);
            ProgressStyle::default_bar()
        })
        .progress_chars("##-");

    pb.set_style(style);
    
    let entries = fs::read_dir(path)?; // Tekrar okuma
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            println!("{}", path.display());
            directories.push(path.display().to_string());
        }
        pb.inc(1); // İlerleme çubuğunu güncelle
    }
    
    pb.finish_with_message("Tamamlandı"); // İlerleme çubuğunu tamamla
    println!("Toplam {} dosya bulundu\nOkuma Başarılı\n\n---------------------------------\n\n", directories.len());
    Ok(directories)
}

fn read_csv(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut keywords = Vec::new();

    for line in reader.lines() {
        let line = line?;
        keywords.push(line);
    }

    Ok(keywords)
}

fn main() -> io::Result<()> {
    let mut path = String::new();
    println!("Klasör Yolu Giriniz: ");
    io::stdin().read_line(&mut path)?;
    let path = path.trim(); // Yeni satır karakterlerini kaldırma

    let mut csv_file_path = String::new();
    println!("CSV dosyasının dizinini giriniz: ");
    io::stdin().read_line(&mut csv_file_path)?;
    let csv_file_path = csv_file_path.trim(); // Yeni satır karakterlerini kaldırma

    let keywords = read_csv(csv_file_path)?;

    let file_paths = sub_dir_list_files(path)?;

    for file_path in file_paths {
        for keyword in &keywords {
            let pattern = keyword.as_bytes();
            kmp_search(pattern, &file_path)?;
            break; // Her dosya için ilk eşleşmeden sonra döngüyü kır
        }
    }

    Ok(())
}
