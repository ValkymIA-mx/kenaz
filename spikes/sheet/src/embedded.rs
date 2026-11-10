use std::{error::Error, fs, path::Path, time::Instant};
use typst::{
    diag::FileError,
    foundations::{Bytes, Datetime, Dict, Duration, Value},
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
    Library, LibraryExt, World,
};
use typst_layout::PagedDocument;

const MAIN: &str = "/templates/recovery-sheet/recovery-sheet.typ";
const DATA: &str = "/spikes/sheet/target/sheet-sample/example-with-qr.json";

fn id(path: &str) -> FileId {
    RootedPath::new(VirtualRoot::Project, VirtualPath::new(path).expect("static path")).intern()
}

struct SampleWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    source: Source,
    source_bytes: Bytes,
    data: Bytes,
}

impl World for SampleWorld {
    fn library(&self) -> &LazyHash<Library> { &self.library }
    fn book(&self) -> &LazyHash<FontBook> { &self.book }
    fn main(&self) -> FileId { id(MAIN) }
    fn source(&self, file: FileId) -> Result<Source, FileError> {
        if file == id(MAIN) { Ok(self.source.clone()) } else { Err(FileError::AccessDenied) }
    }
    fn file(&self, file: FileId) -> Result<Bytes, FileError> {
        if file == id(MAIN) { Ok(self.source_bytes.clone()) }
        else if file == id(DATA) { Ok(self.data.clone()) }
        else { Err(FileError::AccessDenied) }
    }
    fn font(&self, index: usize) -> Option<Font> { self.fonts.get(index).cloned() }
    fn today(&self, _: Option<Duration>) -> Option<Datetime> { None }
}

pub fn render(spike: &Path, template: &Path, data: &Path, output: &Path, lang: &str) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut fonts = Vec::new();
    for name in ["NotoSans-Regular.ttf", "NotoSans-Bold.ttf", "DejaVuSansMono.ttf", "DejaVuSansMono-Bold.ttf"] {
        let bytes = Bytes::new(fs::read(spike.join("fonts").join(name))?);
        fonts.push(Font::new(bytes, 0).ok_or_else(|| format!("invalid font: {name}"))?);
    }
    let text = fs::read_to_string(template)?;
    let mut inputs = Dict::new();
    inputs.insert("data".into(), Value::Str(DATA.into()));
    inputs.insert("lang".into(), Value::Str(lang.into()));
    let world = SampleWorld {
        library: LazyHash::new(Library::builder().with_inputs(inputs).build()),
        book: LazyHash::new(FontBook::from_fonts(&fonts)),
        fonts,
        source: Source::new(id(MAIN), text.clone()),
        source_bytes: Bytes::new(text.into_bytes()),
        data: Bytes::new(fs::read(data)?),
    };
    let result = typst::compile::<PagedDocument>(&world);
    for warning in result.warnings { eprintln!("embedded warning: {warning:?}"); }
    let document = result.output.map_err(|errors| format!("embedded compile failed: {errors:?}"))?;
    let pdf = typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default())
        .map_err(|errors| format!("embedded PDF export failed: {errors:?}"))?;
    let path = output.join(format!("embedded-{lang}.pdf"));
    fs::write(&path, &pdf)?;
    println!("embedded {lang}: {} ms, {} bytes: {}", start.elapsed().as_millis(), pdf.len(), path.display());
    Ok(())
}
