use std::collections::HashMap;
use std::fs;
use std::io::Write;

use regex::Regex;

const EN_PATH: &str = "hpmor-en.html";
const ES_PATH: &str = "hpmor.html";
const HTML_OUT: &str = "hpmor-bilingue.html";
const EPUB_OUT: &str = "hpmor-bilingue.epub";
const COVER_PATH: &str = "hpmor-bilingue-cover.jpg";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("no se puede leer {path}: {e}"))
}

fn strip_asides(html: &str) -> String {
    let re = Regex::new(r"(?s)<aside\b.*?</aside>").unwrap();
    re.replace_all(html, "").into_owned()
}

fn block_sequence(html: &str) -> Vec<String> {
    let re = Regex::new(r"<(p|hr|div|blockquote|ol|ul|h[123])\b").unwrap();
    re.captures_iter(html).map(|c| c[1].to_string()).collect()
}

fn collapse_whitespace(s: &str) -> String {
    let tag_re = Regex::new(r"<[^>]+>").unwrap();
    let ws_re = Regex::new(r"\s+").unwrap();
    ws_re.replace_all(tag_re.replace_all(s, " ").trim(), " ").into_owned()
}

/// Inner HTML of each `<p>`, in document order.
fn spanish_paragraphs(es_no_aside: &str) -> Vec<String> {
    let re = Regex::new(r"(?s)<p\b[^>]*>(.*?)</p>").unwrap();
    re.captures_iter(es_no_aside)
        .map(|c| c[1].to_string())
        .collect()
}

/// Byte offsets of block opening tags, in order.
fn block_hits(html: &str) -> Vec<(String, usize)> {
    let re = Regex::new(r"<(p|hr|div|blockquote|ol|ul|h[123])\b").unwrap();
    re.find_iter(html)
        .map(|m| {
            let tag = m.as_str()[1..]
                .split(|c: char| !c.is_ascii_alphanumeric())
                .next()
                .unwrap_or("")
                .to_string();
            (tag, m.start())
        })
        .collect()
}

/// Byte offsets of each `</p>`, in order (`p` does not nest).
fn p_close_offsets(html: &str) -> Vec<usize> {
    Regex::new(r"</p\s*>")
        .unwrap()
        .find_iter(html)
        .map(|m| m.start())
        .collect()
}

fn heading_text(slice: &str, tag: &str) -> Option<String> {
    let re = Regex::new(&format!(r"(?s)<{tag}\b[^>]*>(.*?)</{tag}>")).unwrap();
    re.captures(slice)
        .map(|c| collapse_whitespace(&c[1]))
        .filter(|t| !t.is_empty())
}

struct Pairing {
    /// (English `<p>` offset, `</p>` offset, global id, Spanish inner HTML)
    notes: Vec<(usize, usize, usize, String)>,
    total_blocks: usize,
    p_pairs: usize,
}

/// Pairs paragraphs by position and decides which ones get a note.
/// `es_paras` holds only the Spanish `<p>` elements, in order: the k-th
/// English `<p>` is paired with the k-th Spanish one (the block-sequence
/// assertion already guaranteed that both documents have the same blocks
/// in the same order).
fn pair(en_no_aside: &str, es_paras: &[String]) -> Pairing {
    let hits = block_hits(en_no_aside);
    let closes = p_close_offsets(en_no_aside);

    let mut notes = Vec::new();
    let mut p_pairs = 0usize;

    for (tag, open_off) in &hits {
        if tag != "p" {
            continue;
        }
        let close_off = *closes.get(p_pairs)
            .unwrap_or_else(|| panic!("`<p>` sin cierre nº {p_pairs}"));
        let es_inner = &es_paras[p_pairs];
        p_pairs += 1;

        if !collapse_whitespace(es_inner).is_empty() {
            let n = notes.len() + 1;
            notes.push((*open_off, close_off, n, es_inner.clone()));
        }
    }

    assert_eq!(p_pairs, closes.len(), "cierres de `<p>` sobrantes");
    assert_eq!(closes.len(), es_paras.len(), "los `<p>` no coinciden en número");
    Pairing { notes, total_blocks: hits.len(), p_pairs }
}

enum Mode {
    Html,
    Epub,
}

/// Applies the grafts to the text, processing offsets in descending order.
/// `note_targets` (EPUB only) maps each note to its translations file;
/// links are cross-file: `notasNNN.xhtml#es-nota-N`.
fn inject(
    html: &str,
    notes: &[(usize, usize, usize, String)],
    mode: Mode,
    note_targets: &HashMap<usize, String>,
) -> String {
    let mut out = html.to_string();
    for (open_off, close_off, n, inner) in notes.iter().rev() {
        match mode {
            Mode::Html => {
                // CSS anchor on the paragraph; popover positioned right above it.
                out.insert_str(
                    *close_off,
                    &format!(
                        "<button type=\"button\" class=\"tr-note\" id=\"en-nota-{n}\" \
                         popovertarget=\"es-nota-{n}\">*</button><span class=\"tr-pop\" \
                         popover=\"auto\" id=\"es-nota-{n}\" lang=\"es\" \
                         style=\"position-anchor: --tr-{n}\">{inner}</span>"
                    ),
                );
                // `open_off + 2` lands right after `<p`, with or without attributes.
                out.insert_str(
                    open_off + 2,
                    &format!(" style=\"anchor-name: --tr-{n}\""),
                );
            }
            Mode::Epub => {
                let target = note_targets
                    .get(n)
                    .unwrap_or_else(|| panic!("nota {n} sin fichero de destino"));
                out.insert_str(
                    *close_off,
                    &format!(
                        "<a class=\"tr-note\" id=\"en-nota-{n}\" epub:type=\"noteref\" \
                         href=\"{target}#es-nota-{n}\"><sup>*</sup></a>"
                    ),
                );
            }
        }
    }
    out
}

const BILINGUE_CSS: &str = r#"
/* bilingue.css - translation notes */
.tr-note {
  background: none;
  border: none;
  padding: 0;
  margin: 0;
  font: inherit;
  font-size: 0.7em;
  vertical-align: super;
  line-height: 0;
  color: #8a8a8a;
  text-decoration: none;
  cursor: pointer;
}
span.tr-pop {
  border: none;
  border-left: 3px solid #b9b2a5;
  background-color: #f5f3ee;
  color: #333;
  font-size: 0.92em;
  padding: 0.6em 0.9em;
  margin: auto;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.18);
  max-width: 34rem;
  text-align: left;
}
/* With CSS Anchor Positioning: popover right above its paragraph,
   at full paragraph width. Without support, it stays centered (rule above). */
@supports (anchor-name: --a) {
  span.tr-pop {
    position-area: center top;
    align-self: end;
    justify-self: stretch;
    margin: 0 0 0.35em 0;
    max-width: none;
  }
}
section.traducciones h2 {
  font-size: 1.1em;
  color: #666;
  margin-top: 2.5em;
}
section.traducciones p {
  font-size: 0.9em;
  color: #444;
}
section.traducciones a.backlink {
  text-decoration: none;
  color: #8a8a8a;
}
"#;

// ---------------------------------------------------------------- EPUB ---

fn wrap_xhtml(title: &str, body_content: &str, lang: &str) -> String {
    // The source is HTML5 despite being declared XHTML: close void elements
    // and replace HTML entities not defined in XML.
    let void_re = Regex::new(r"<(br|hr)>").unwrap();
    let body = void_re.replace_all(body_content, "<$1/>");
    let body = body.replace("&nbsp;", "&#160;");
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
         <!DOCTYPE html>\n\
         <html xmlns=\"http://www.w3.org/1999/xhtml\" \
         xmlns:epub=\"http://www.idpf.org/2007/ops\" xml:lang=\"{lang}\" lang=\"{lang}\">\n\
         <head><meta charset=\"utf-8\"/><title>{}</title>\n\
         <link rel=\"stylesheet\" type=\"text/css\" href=\"style.css\"/>\n\
         </head><body>{}</body></html>\n",
        title,
        body
    )
}

fn starts_with_h1(slice: &str) -> bool {
    match (slice.find("<h1"), slice.find("<h2")) {
        (Some(h1), Some(h2)) => h1 < h2,
        (Some(_), None) => true,
        _ => false,
    }
}

/// Optional front-matter credit page (Spanish edition only).
struct CreditsPage {
    nav_label: String,
    doc_title: String,
    body: String,
}

/// Everything that differs between the bilingual EPUB and the plain
/// Spanish edition.
struct EpubOpts<'a> {
    title: &'a str,
    author: &'a str,
    lang: &'a str,
    nav_title: &'a str,
    cover_label: &'a str,
    cover_jpeg: Option<&'a [u8]>,
    credits: Option<CreditsPage>,
    contributor: Option<&'a str>,
    extra_css: &'a str,
    bookid: &'a str,
}

/// Current UTC time as `YYYY-MM-DDTHH:MM:SSZ` (for dcterms:modified).
fn utc_now_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let civil = |z: i64| -> (i64, u32, u32) {
        let z = z + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z.rem_euclid(146_097);
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        (if m <= 2 { y + 1 } else { y }, m, d)
    };
    let (y, mo, d) = civil((secs / 86_400) as i64);
    let (hh, mm, ss) = (
        (secs % 86_400) / 3600,
        (secs % 3600) / 60,
        secs % 60,
    );
    format!("{y:04}-{mo:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn build_epub(
    src_html: &str,
    body_start: usize,
    body_end: usize,
    cut_points: &[usize],
    pairing: Option<&Pairing>,
    opts: &EpubOpts,
) -> Vec<u8> {
    let EpubOpts {
        title,
        author,
        lang,
        nav_title,
        cover_label,
        cover_jpeg,
        credits,
        contributor,
        extra_css,
        bookid,
    } = opts;
    // Body cuts: cover/front matter + one per `<h2>`.
    let mut bounds: Vec<usize> = vec![body_start];
    bounds.extend_from_slice(cut_points);
    bounds.push(body_end);

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for w in bounds.windows(2) {
        ranges.push((w[0], w[1]));
    }

    // Distribute notes per cut (not injected yet). A plain edition (no
    // pairing) gets no markers and no notes files.
    let mut own_per_slice: Vec<Vec<(usize, usize, usize, String)>> =
        Vec::with_capacity(ranges.len());
    for &(a, b) in &ranges {
        let own: Vec<(usize, usize, usize, String)> = pairing
            .map(|p| {
                p.notes
                    .iter()
                    .filter(|(off, _, _, _)| *off >= a && *off < b)
                    .map(|(off, close, n, inner)| {
                        (off - a, close - a, *n, inner.clone())
                    })
                    .collect()
            })
            .unwrap_or_default();
        own_per_slice.push(own);
    }

    // Translations file for each note: all at the end of the book.
    let mut note_targets: HashMap<usize, String> = HashMap::new();
    for (idx, own) in own_per_slice.iter().enumerate() {
        if own.is_empty() {
            continue;
        }
        let name = format!("notas{idx:03}.xhtml");
        for (_, _, n, _) in own {
            note_targets.insert(*n, name.clone());
        }
    }

    // Inject only the markers into the chapters.
    let mut slices: Vec<String> = Vec::with_capacity(ranges.len());
    for (own, &(a, b)) in own_per_slice.iter().zip(&ranges) {
        if pairing.is_some() {
            slices.push(inject(&src_html[a..b], own, Mode::Epub, &note_targets));
        } else {
            slices.push(src_html[a..b].to_string());
        }
    }

    // Chapter XHTML files.
    let mut files: Vec<(String, String)> = Vec::new();
    let mut titles: Vec<String> = Vec::with_capacity(slices.len());
    for (idx, slice) in slices.iter().enumerate() {
        let chap_title = heading_text(slice, "h2")
            .or_else(|| heading_text(slice, "h1"))
            .unwrap_or_else(|| title.to_string());
        titles.push(chap_title.clone());
        files.push((
            format!("sec{idx:03}.xhtml"),
            wrap_xhtml(&chap_title, slice, lang),
        ));
    }

    // Translation files, one per chapter with notes, at the end of the book.
    let mut notes_files: Vec<(String, String)> = Vec::new();
    let mut notes_nav: Vec<(String, String)> = Vec::new(); // (href, title)
    for (idx, own) in own_per_slice.iter().enumerate() {
        if own.is_empty() {
            continue;
        }
        let name = format!("notas{idx:03}.xhtml");
        let chap_title = &titles[idx];
        let mut body =
            format!("<section class=\"traducciones\"><h2 lang=\"es\">Traducción — {chap_title}</h2>");
        for (_, _, n, inner) in own {
            body.push_str(&format!(
                "<p id=\"es-nota-{n}\" lang=\"es\" epub:type=\"footnote\">{inner} \
                 <a class=\"backlink\" href=\"sec{idx:03}.xhtml#en-nota-{n}\" \
                 epub:type=\"backlink\">\u{21A9}</a></p>"
            ));
        }
        body.push_str("</section>");
        let doc_title = format!("Traducción — {chap_title}");
        notes_files.push((name.clone(), wrap_xhtml(&doc_title, &body, lang)));
        notes_nav.push((name, doc_title));
    }

    // Cover page (if the image exists).
    let cover_page = cover_jpeg.map(|_| {
        let body = format!(
            "<div style=\"text-align:center;margin:0;padding:0\">\
             <img src=\"cover.jpg\" alt=\"{title}\" \
             style=\"max-width:100%;max-height:100%;height:auto\"/></div>",
            title = title
        );
        (
            "cover.xhtml".to_string(),
            wrap_xhtml(cover_label, &body, lang),
        )
    });

    // Credit page (Spanish edition).
    let credits_file = credits.as_ref().map(|c| {
        (
            c.nav_label.clone(),
            "creditos.xhtml".to_string(),
            wrap_xhtml(&c.doc_title, &c.body, lang),
        )
    });

    // nav.xhtml, two levels: book title + arcs with their chapters.
    let mut nav = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
         <!DOCTYPE html>\n\
         <html xmlns=\"http://www.w3.org/1999/xhtml\" \
         xmlns:epub=\"http://www.idpf.org/2007/ops\" xml:lang=\"{lang}\" lang=\"{lang}\">\n\
         <head><meta charset=\"utf-8\"/><title>{nav_title}</title></head>\n\
         <body><nav epub:type=\"toc\" id=\"toc\"><h1>{nav_title}</h1><ol>\n"
    );
    if let Some((name, _)) = &cover_page {
        nav.push_str(&format!("<li><a href=\"{name}\">{cover_label}</a></li>\n"));
    }
    if let Some((label, name, _)) = &credits_file {
        nav.push_str(&format!("<li><a href=\"{name}\">{label}</a></li>\n"));
    }
    nav.push_str(&format!(
        "<li><a href=\"sec000.xhtml\">{}</a></li>\n",
        title
    ));

    let mut i = 1usize;
    while i < slices.len() {
        if starts_with_h1(&slices[i]) {
            let arc_title =
                heading_text(&slices[i], "h1").unwrap_or_else(|| format!("Section {i}"));
            nav.push_str(&format!(
                "<li><a href=\"sec{i:03}.xhtml\">{}</a><ol>\n",
                arc_title
            ));
            let arc_start = i;
            i += 1;
            while i < slices.len() && !starts_with_h1(&slices[i]) {
                let t = heading_text(&slices[i], "h2")
                    .unwrap_or_else(|| format!("Section {i}"));
                nav.push_str(&format!(
                    "<li><a href=\"sec{i:03}.xhtml\">{}</a></li>\n",
                    t
                ));
                i += 1;
            }
            let _ = arc_start;
            nav.push_str("</ol></li>\n");
        } else {
            let t =
                heading_text(&slices[i], "h2").unwrap_or_else(|| format!("Section {i}"));
            nav.push_str(&format!(
                "<li><a href=\"sec{i:03}.xhtml\">{}</a></li>\n",
                t
            ));
            i += 1;
        }
    }

    // Final TOC entry: all translations.
    if !notes_nav.is_empty() {
        nav.push_str(&format!(
            "<li><a href=\"{}\">Traducciones</a><ol>\n",
            notes_nav[0].0
        ));
        for (href, t) in &notes_nav {
            nav.push_str(&format!("<li><a href=\"{href}\">{t}</a></li>\n"));
        }
        nav.push_str("</ol></li>\n");
    }
    nav.push_str("</ol></nav></body></html>\n");

    // style.css = original stylesheet + bilingual rules.
    let style_re = Regex::new(r"(?s)<style>(.*?)</style>").unwrap();
    let original_css = style_re
        .captures(src_html)
        .map(|c| c[1].to_string())
        .unwrap_or_default();

    // content.opf
    let mut manifest = String::from(
        "<item id=\"nav\" href=\"nav.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/>\n\
         <item id=\"css\" href=\"style.css\" media-type=\"text/css\"/>\n",
    );
    let mut spine = String::new();
    if cover_page.is_some() {
        manifest.push_str(
            "<item id=\"cover\" href=\"cover.xhtml\" media-type=\"application/xhtml+xml\"/>\n\
             <item id=\"cover-image\" href=\"cover.jpg\" media-type=\"image/jpeg\" properties=\"cover-image\"/>\n",
        );
        spine.push_str("<itemref idref=\"cover\"/>\n");
    }
    if let Some((_, name, _)) = &credits_file {
        manifest.push_str(&format!(
            "<item id=\"creditos\" href=\"{name}\" media-type=\"application/xhtml+xml\"/>\n"
        ));
        spine.push_str("<itemref idref=\"creditos\"/>\n");
    }
    for (name, _) in files.iter().chain(notes_files.iter()) {
        let id = name.trim_end_matches(".xhtml");
        manifest.push_str(&format!(
            "<item id=\"{id}\" href=\"{name}\" media-type=\"application/xhtml+xml\"/>\n"
        ));
        spine.push_str(&format!("<itemref idref=\"{id}\"/>\n"));
    }
    let contributor_line = contributor
        .map(|c| format!("\n<dc:contributor>{c}</dc:contributor>"))
        .unwrap_or_default();
    let opf = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
         <package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"bookid\">\n\
         <metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\">\n\
         <dc:identifier id=\"bookid\">{bookid}</dc:identifier>\n\
         <dc:title>{title}</dc:title>\n\
         <dc:creator>{author}</dc:creator>{contributor_line}\n\
         <dc:language>{lang}</dc:language>\n\
         <meta property=\"dcterms:modified\">{}</meta>\n\
         </metadata>\n<manifest>{manifest}</manifest>\n<spine>{spine}</spine>\n</package>\n",
         utc_now_iso(),
    );

    let container = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
         <container version=\"1.0\" xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\">\n\
         <rootfiles><rootfile full-path=\"OEBPS/content.opf\" \
         media-type=\"application/oebps-package+xml\"/></rootfiles>\n</container>\n";

    // ZIP: mimetype first and uncompressed.
    let buf = std::io::Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);
    let stored = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    let deflated = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    zip.start_file("mimetype", stored).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();
    zip.start_file("META-INF/container.xml", deflated).unwrap();
    zip.write_all(container.as_bytes()).unwrap();

    if let (Some((_, page)), Some(jpeg)) = (&cover_page, cover_jpeg) {
        zip.start_file("OEBPS/cover.jpg", deflated).unwrap();
        zip.write_all(jpeg).unwrap();
    }

    let mut add = |name: &str, data: &str| {
        zip.start_file(name, deflated).unwrap();
        zip.write_all(data.as_bytes()).unwrap();
    };
    add("OEBPS/content.opf", &opf);
    add("OEBPS/nav.xhtml", &nav);
    add("OEBPS/style.css", &format!("{original_css}\n{extra_css}"));
    if let Some((_, page)) = &cover_page {
        add("OEBPS/cover.xhtml", page);
    }
    if let Some((_, _, page)) = &credits_file {
        add("OEBPS/creditos.xhtml", page);
    }
    for (name, content) in files.iter().chain(notes_files.iter()) {
        add(&format!("OEBPS/{name}"), content);
    }
    zip.finish().unwrap().into_inner()
}

fn main() {
    let en = read(EN_PATH);
    let es = read(ES_PATH);

    let en_no_aside = strip_asides(&en);
    let es_no_aside = strip_asides(&es);

    // Hard assertion: identical block sequences.
    let seq_en = block_sequence(&en_no_aside);
    let seq_es = block_sequence(&es_no_aside);
    assert_eq!(
        seq_en, seq_es,
        "secuencia de bloques divergente: {} vs {}",
        seq_en.len(),
        seq_es.len()
    );

    let es_paras = spanish_paragraphs(&es_no_aside);
    let pairing = pair(&en_no_aside, &es_paras);

    println!(
        "bloques: {} · parejas <p>: {} · notas insertadas: {}",
        pairing.total_blocks,
        pairing.p_pairs,
        pairing.notes.len()
    );

    // --- HTML ---
    let empty_targets = HashMap::new();
    let mut html_out = inject(&en, &pairing.notes, Mode::Html, &empty_targets);
    let css_injection = format!("\n<style>{BILINGUE_CSS}</style>\n");
    let head_close = html_out
        .find("</head>")
        .unwrap_or_else(|| panic!("sin </head> en el inglés"));
    html_out.insert_str(head_close, &css_injection);
    fs::write(HTML_OUT, &html_out).unwrap();
    println!("escrito {HTML_OUT}");

    // --- EPUB helpers shared by both editions ---
    let body_bounds = |html: &str| -> (usize, usize) {
        let start = html
            .find("<body")
            .map(|p| html[p..].find('>').unwrap() + p + 1)
            .expect("sin <body>");
        (start, html.find("</body>").expect("sin </body>"))
    };
    // Cuts at each `<h2>` and also at arc `<h1>`s (those after the first
    // `<h2>`; the cover `<h1>` lives inside <header> and stays).
    let chapter_cuts = |html: &str| -> Vec<usize> {
        let (bs, be) = body_bounds(html);
        let hit_re = Regex::new(r"<h([12])\b").unwrap();
        let mut cuts: Vec<usize> = Vec::new();
        let mut seen_h2 = false;
        for m in hit_re.captures_iter(&html[bs..be]) {
            let off = m.get(0).unwrap().start() + bs;
            if &m[1] == "2" {
                seen_h2 = true;
                cuts.push(off);
            } else if seen_h2 {
                cuts.push(off);
            }
        }
        cuts.sort_unstable();
        cuts.dedup();
        cuts
    };
    let doc_title = |html: &str| -> String {
        let re = Regex::new(r"(?s)<title>(.*?)</title>").unwrap();
        collapse_whitespace(
            re.captures(html)
                .map(|c| c[1].to_string())
                .unwrap_or_default()
                .as_str(),
        )
    };
    let doc_author = |html: &str| -> String {
        let re = Regex::new(r#"<meta name="author" content="([^"]*)""#).unwrap();
        re.captures(html)
            .map(|c| c[1].to_string())
            .unwrap_or_else(|| "Eliezer Yudkowsky".into())
    };
    let load_cover = |path: &str| -> Option<Vec<u8>> {
        match fs::read(path) {
            Ok(bytes) => {
                println!("portada: {path} ({} bytes)", bytes.len());
                Some(bytes)
            }
            Err(_) => {
                println!("aviso: sin portada ({path} no encontrado)");
                None
            }
        }
    };

    // --- Bilingual EPUB ---
    let title = doc_title(&en);
    let author = doc_author(&en);
    let (en_start, en_end) = body_bounds(&en);
    let en_cuts = chapter_cuts(&en);
    assert_eq!(
        en_cuts.len(),
        131,
        "esperaba 131 cortes en el inglés (125 h2 + 6 h1 de arco)"
    );
    let cover_jpeg = load_cover(COVER_PATH);
    let opts = EpubOpts {
        title: &title,
        author: &author,
        lang: "en",
        nav_title: "Table of Contents",
        cover_label: "Cover",
        cover_jpeg: cover_jpeg.as_deref(),
        credits: None,
        contributor: None,
        extra_css: BILINGUE_CSS,
        bookid: "urn:uuid:hpmor-bilingue-0001",
    };
    let epub_bytes = build_epub(&en, en_start, en_end, &en_cuts, Some(&pairing), &opts);
    fs::write(EPUB_OUT, &epub_bytes).unwrap();
    println!("escrito {EPUB_OUT}");

    // --- Spanish-only edition ---
    const ES_EPUB_OUT: &str = "hpmor.epub";
    const ES_COVER_PATH: &str = "hpmor-portada.jpg";
    let es_title = doc_title(&es);
    let es_author = doc_author(&es);
    let (es_start, es_end) = body_bounds(&es);
    let es_cuts = chapter_cuts(&es);
    assert_eq!(
        es_cuts.len(),
        131,
        "esperaba 131 cortes en el español (125 h2 + 6 h1 de arco)"
    );
    let es_cover = load_cover(ES_COVER_PATH);
    let credits = CreditsPage {
        nav_label: "Créditos".into(),
        doc_title: "Créditos".into(),
        body: "<section class=\"creditos\">\
               <p>Traducción al español: Marc Morente.</p>\
               <p>Traducción realizada íntegramente por inteligencia artificial.</p>\
               </section>"
            .into(),
    };
    let es_opts = EpubOpts {
        title: &es_title,
        author: &es_author,
        lang: "es",
        nav_title: "Índice",
        cover_label: "Portada",
        cover_jpeg: es_cover.as_deref(),
        credits: Some(credits),
        contributor: Some(
            "Marc Morente — traducción íntegramente por inteligencia artificial",
        ),
        extra_css: "",
        bookid: "urn:uuid:hpmor-es-0001",
    };
    let es_epub = build_epub(&es, es_start, es_end, &es_cuts, None, &es_opts);
    fs::write(ES_EPUB_OUT, es_epub).unwrap();
    println!("escrito {ES_EPUB_OUT}");
}
