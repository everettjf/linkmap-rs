//
// written by everettjf
// email : everettjf@live.com
// created at 2022-01-02
//

mod demangle;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    u64,
};

use anyhow::Result;
use demangle::demangle_symbol;

#[derive(Debug)]
pub struct ObjectFile {
    pub index: u32,
    pub path: String,
}

#[derive(Debug)]
pub struct SectionObject {
    pub address: u64,
    pub size: u64,
    pub segment: String,
    pub section: String,
}
#[derive(Debug)]
pub struct SymbolObject {
    pub address: u64,
    pub size: u64,
    pub file_index: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct LinkMap {
    pub path: String,
    pub arch: String,
    pub object_files: Vec<ObjectFile>,
    pub sections: Vec<SectionObject>,
    pub symbols: Vec<SymbolObject>,
    pub dead_stripped_symbols: Vec<SymbolObject>,
}

enum ParsingPhase {
    Unknown,
    ObjectFiles,
    SectionsHeader,
    Sections,
    SymbolsHeader,
    Symbols,
    DeadStrippedSymbolsHeader,
    DeadStrippedSymbols,
}

pub fn parse_linkmap(path: &str, demangle: bool) -> Result<LinkMap, anyhow::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut linkmap = LinkMap {
        path: "".to_string(),
        arch: "".to_string(),
        object_files: vec![],
        sections: vec![],
        symbols: vec![],
        dead_stripped_symbols: vec![],
    };

    static FLAG_PATH: &str = "# Path: ";
    static FLAG_ARCH: &str = "# Arch: ";
    static FLAG_OBJECT_FILES: &str = "# Object files:";

    static FLAG_SECTIONS: &str = "# Sections:";
    static FLAG_SYMBOLS: &str = "# Symbols:";
    static FLAG_DEAD_STRIPPED_SYMBOLS: &str = "# Dead Stripped Symbols:";

    let mut parsing_phase = ParsingPhase::Unknown;

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        match parsing_phase {
            ParsingPhase::ObjectFiles => {
                let mut parts = line.splitn(2, "] ");
                if let (Some(first), Some(file_path)) = (parts.next(), parts.next()) {
                    let index_string = first[1..].trim();
                    if let Ok(index) = index_string.parse::<u32>() {
                        // println!("index = {}, file_path = {}", index, file_path);
                        linkmap.object_files.push(ObjectFile {
                            index: index,
                            path: file_path.to_string(),
                        });
                    } else {
                        println!("Failed parse index to u32 : {}", index_string);
                    }
                }
            }
            ParsingPhase::Sections => {
                let mut parts = line.splitn(4, "\t");
                if let (
                    Some(address_hexstr),
                    Some(size_hexstr),
                    Some(segment_name),
                    Some(section_name),
                ) = (parts.next(), parts.next(), parts.next(), parts.next())
                {
                    let address_hexstr_without_prefix = address_hexstr.trim_start_matches("0x");
                    let size_hexstr_without_prefix = size_hexstr.trim_start_matches("0x");
                    if let (Ok(address), Ok(size)) = (
                        u64::from_str_radix(address_hexstr_without_prefix, 16),
                        u64::from_str_radix(size_hexstr_without_prefix, 16),
                    ) {
                        // println!(
                        //     "address = {}, size = {}, segment = {}, section = {}",
                        //     address, size, segment_name, section_name
                        // );
                        linkmap.sections.push(SectionObject {
                            address: address,
                            size: size,
                            segment: segment_name.to_string(),
                            section: section_name.to_string(),
                        });
                    } else {
                        println!(
                            "Failed parse address {} or size {}",
                            address_hexstr, size_hexstr
                        );
                    }
                }
            }
            ParsingPhase::Symbols => {
                let mut parts = line.splitn(3, "\t");
                if let (Some(address_hexstr), Some(size_hexstr), Some(text)) =
                    ((parts.next()), (parts.next()), (parts.next()))
                {
                    let mut text_parts = text.splitn(2, "] ");

                    if let (Some(index_string), Some(symbol_name)) =
                        (text_parts.next(), text_parts.next())
                    {
                        let address_hexstr_without_prefix = address_hexstr.trim_start_matches("0x");
                        let size_hexstr_without_prefix = size_hexstr.trim_start_matches("0x");
                        if let (Ok(address), Ok(size)) = (
                            u64::from_str_radix(address_hexstr_without_prefix, 16),
                            u64::from_str_radix(size_hexstr_without_prefix, 16),
                        ) {
                            let index_string = index_string[1..].trim();
                            if let Ok(index) = index_string.parse::<u32>() {
                                let symbol = if demangle {
                                    demangle_symbol(symbol_name)
                                } else {
                                    symbol_name.to_string()
                                };
                                linkmap.symbols.push(SymbolObject {
                                    address: address,
                                    size: size,
                                    file_index: index,
                                    name: symbol,
                                });
                            } else {
                                println!("Failed parse index to u32 : {}", index_string);
                            }
                        } else {
                            println!(
                                "Failed parse address {} or size {}",
                                address_hexstr, size_hexstr
                            );
                        }
                    }
                }
            }
            ParsingPhase::DeadStrippedSymbols => {
                let mut parts = line.splitn(3, "\t");
                if let (Some(address_hexstr), Some(size_hexstr), Some(text)) =
                    ((parts.next()), (parts.next()), (parts.next()))
                {
                    let mut text_parts = text.splitn(2, "] ");
                    if let (Some(index_string), Some(symbol_name)) =
                        (text_parts.next(), text_parts.next())
                    {
                        let size_hexstr_without_prefix = size_hexstr.trim_start_matches("0x");
                        if let Ok(size) = u64::from_str_radix(size_hexstr_without_prefix, 16) {
                            let index_string = index_string[1..].trim();
                            if let Ok(index) = index_string.parse::<u32>() {
                                let symbol = if demangle {
                                    demangle_symbol(symbol_name)
                                } else {
                                    symbol_name.to_string()
                                };
                                linkmap.dead_stripped_symbols.push(SymbolObject {
                                    address: 0,
                                    size: size,
                                    file_index: index,
                                    name: symbol,
                                });
                            } else {
                                println!("Failed parse index to u32 : {}", index_string);
                            }
                        } else {
                            println!(
                                "Failed parse address {} or size {}",
                                address_hexstr, size_hexstr
                            );
                        }
                    }
                }
            }

            ParsingPhase::Unknown => {}
            ParsingPhase::SectionsHeader => {
                parsing_phase = ParsingPhase::Sections;
                // move to next line
                continue;
            }
            ParsingPhase::SymbolsHeader => {
                parsing_phase = ParsingPhase::Symbols;
                // move to next line
                continue;
            }
            ParsingPhase::DeadStrippedSymbolsHeader => {
                parsing_phase = ParsingPhase::DeadStrippedSymbols;
                // move to next line
                continue;
            }
        }

        if line.starts_with("#") {
            if line.starts_with(FLAG_PATH) {
                let right = FLAG_PATH.len();
                linkmap.path = line[right..].to_string();
            } else if line.starts_with(FLAG_ARCH) {
                let right = FLAG_ARCH.len();
                linkmap.arch = line[right..].to_string();
            } else if line.starts_with(FLAG_OBJECT_FILES) {
                parsing_phase = ParsingPhase::ObjectFiles;
            } else if line.starts_with(FLAG_SECTIONS) {
                parsing_phase = ParsingPhase::SectionsHeader;
            } else if line.starts_with(FLAG_SYMBOLS) {
                parsing_phase = ParsingPhase::SymbolsHeader;
            } else if line.starts_with(FLAG_DEAD_STRIPPED_SYMBOLS) {
                parsing_phase = ParsingPhase::DeadStrippedSymbolsHeader;
            } else {
                parsing_phase = ParsingPhase::Unknown;
                println!("Unknow line format : {}", line);
            }
        }
    }

    Ok(linkmap)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
