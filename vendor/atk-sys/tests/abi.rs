// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["atk"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "AtkActionIface",
        Layout {
            size: size_of::<AtkActionIface>(),
            alignment: align_of::<AtkActionIface>(),
        },
    ),
    (
        "AtkAttribute",
        Layout {
            size: size_of::<AtkAttribute>(),
            alignment: align_of::<AtkAttribute>(),
        },
    ),
    (
        "AtkAttributeSet",
        Layout {
            size: size_of::<AtkAttributeSet>(),
            alignment: align_of::<AtkAttributeSet>(),
        },
    ),
    (
        "AtkComponentIface",
        Layout {
            size: size_of::<AtkComponentIface>(),
            alignment: align_of::<AtkComponentIface>(),
        },
    ),
    (
        "AtkCoordType",
        Layout {
            size: size_of::<AtkCoordType>(),
            alignment: align_of::<AtkCoordType>(),
        },
    ),
    (
        "AtkDocumentIface",
        Layout {
            size: size_of::<AtkDocumentIface>(),
            alignment: align_of::<AtkDocumentIface>(),
        },
    ),
    (
        "AtkEditableTextIface",
        Layout {
            size: size_of::<AtkEditableTextIface>(),
            alignment: align_of::<AtkEditableTextIface>(),
        },
    ),
    (
        "AtkGObjectAccessible",
        Layout {
            size: size_of::<AtkGObjectAccessible>(),
            alignment: align_of::<AtkGObjectAccessible>(),
        },
    ),
    (
        "AtkGObjectAccessibleClass",
        Layout {
            size: size_of::<AtkGObjectAccessibleClass>(),
            alignment: align_of::<AtkGObjectAccessibleClass>(),
        },
    ),
    (
        "AtkHyperlink",
        Layout {
            size: size_of::<AtkHyperlink>(),
            alignment: align_of::<AtkHyperlink>(),
        },
    ),
    (
        "AtkHyperlinkClass",
        Layout {
            size: size_of::<AtkHyperlinkClass>(),
            alignment: align_of::<AtkHyperlinkClass>(),
        },
    ),
    (
        "AtkHyperlinkImplIface",
        Layout {
            size: size_of::<AtkHyperlinkImplIface>(),
            alignment: align_of::<AtkHyperlinkImplIface>(),
        },
    ),
    (
        "AtkHyperlinkStateFlags",
        Layout {
            size: size_of::<AtkHyperlinkStateFlags>(),
            alignment: align_of::<AtkHyperlinkStateFlags>(),
        },
    ),
    (
        "AtkHypertextIface",
        Layout {
            size: size_of::<AtkHypertextIface>(),
            alignment: align_of::<AtkHypertextIface>(),
        },
    ),
    (
        "AtkImageIface",
        Layout {
            size: size_of::<AtkImageIface>(),
            alignment: align_of::<AtkImageIface>(),
        },
    ),
    (
        "AtkKeyEventStruct",
        Layout {
            size: size_of::<AtkKeyEventStruct>(),
            alignment: align_of::<AtkKeyEventStruct>(),
        },
    ),
    (
        "AtkKeyEventType",
        Layout {
            size: size_of::<AtkKeyEventType>(),
            alignment: align_of::<AtkKeyEventType>(),
        },
    ),
    (
        "AtkLayer",
        Layout {
            size: size_of::<AtkLayer>(),
            alignment: align_of::<AtkLayer>(),
        },
    ),
    (
        "AtkMisc",
        Layout {
            size: size_of::<AtkMisc>(),
            alignment: align_of::<AtkMisc>(),
        },
    ),
    (
        "AtkMiscClass",
        Layout {
            size: size_of::<AtkMiscClass>(),
            alignment: align_of::<AtkMiscClass>(),
        },
    ),
    (
        "AtkNoOpObject",
        Layout {
            size: size_of::<AtkNoOpObject>(),
            alignment: align_of::<AtkNoOpObject>(),
        },
    ),
    (
        "AtkNoOpObjectClass",
        Layout {
            size: size_of::<AtkNoOpObjectClass>(),
            alignment: align_of::<AtkNoOpObjectClass>(),
        },
    ),
    (
        "AtkNoOpObjectFactory",
        Layout {
            size: size_of::<AtkNoOpObjectFactory>(),
            alignment: align_of::<AtkNoOpObjectFactory>(),
        },
    ),
    (
        "AtkNoOpObjectFactoryClass",
        Layout {
            size: size_of::<AtkNoOpObjectFactoryClass>(),
            alignment: align_of::<AtkNoOpObjectFactoryClass>(),
        },
    ),
    (
        "AtkObject",
        Layout {
            size: size_of::<AtkObject>(),
            alignment: align_of::<AtkObject>(),
        },
    ),
    (
        "AtkObjectClass",
        Layout {
            size: size_of::<AtkObjectClass>(),
            alignment: align_of::<AtkObjectClass>(),
        },
    ),
    (
        "AtkObjectFactory",
        Layout {
            size: size_of::<AtkObjectFactory>(),
            alignment: align_of::<AtkObjectFactory>(),
        },
    ),
    (
        "AtkObjectFactoryClass",
        Layout {
            size: size_of::<AtkObjectFactoryClass>(),
            alignment: align_of::<AtkObjectFactoryClass>(),
        },
    ),
    (
        "AtkPlug",
        Layout {
            size: size_of::<AtkPlug>(),
            alignment: align_of::<AtkPlug>(),
        },
    ),
    (
        "AtkPlugClass",
        Layout {
            size: size_of::<AtkPlugClass>(),
            alignment: align_of::<AtkPlugClass>(),
        },
    ),
    (
        "AtkPropertyValues",
        Layout {
            size: size_of::<AtkPropertyValues>(),
            alignment: align_of::<AtkPropertyValues>(),
        },
    ),
    (
        "AtkRectangle",
        Layout {
            size: size_of::<AtkRectangle>(),
            alignment: align_of::<AtkRectangle>(),
        },
    ),
    (
        "AtkRegistry",
        Layout {
            size: size_of::<AtkRegistry>(),
            alignment: align_of::<AtkRegistry>(),
        },
    ),
    (
        "AtkRegistryClass",
        Layout {
            size: size_of::<AtkRegistryClass>(),
            alignment: align_of::<AtkRegistryClass>(),
        },
    ),
    (
        "AtkRelation",
        Layout {
            size: size_of::<AtkRelation>(),
            alignment: align_of::<AtkRelation>(),
        },
    ),
    (
        "AtkRelationClass",
        Layout {
            size: size_of::<AtkRelationClass>(),
            alignment: align_of::<AtkRelationClass>(),
        },
    ),
    (
        "AtkRelationSet",
        Layout {
            size: size_of::<AtkRelationSet>(),
            alignment: align_of::<AtkRelationSet>(),
        },
    ),
    (
        "AtkRelationSetClass",
        Layout {
            size: size_of::<AtkRelationSetClass>(),
            alignment: align_of::<AtkRelationSetClass>(),
        },
    ),
    (
        "AtkRelationType",
        Layout {
            size: size_of::<AtkRelationType>(),
            alignment: align_of::<AtkRelationType>(),
        },
    ),
    (
        "AtkRole",
        Layout {
            size: size_of::<AtkRole>(),
            alignment: align_of::<AtkRole>(),
        },
    ),
    (
        "AtkScrollType",
        Layout {
            size: size_of::<AtkScrollType>(),
            alignment: align_of::<AtkScrollType>(),
        },
    ),
    (
        "AtkSelectionIface",
        Layout {
            size: size_of::<AtkSelectionIface>(),
            alignment: align_of::<AtkSelectionIface>(),
        },
    ),
    (
        "AtkSocket",
        Layout {
            size: size_of::<AtkSocket>(),
            alignment: align_of::<AtkSocket>(),
        },
    ),
    (
        "AtkSocketClass",
        Layout {
            size: size_of::<AtkSocketClass>(),
            alignment: align_of::<AtkSocketClass>(),
        },
    ),
    (
        "AtkState",
        Layout {
            size: size_of::<AtkState>(),
            alignment: align_of::<AtkState>(),
        },
    ),
    (
        "AtkStateSet",
        Layout {
            size: size_of::<AtkStateSet>(),
            alignment: align_of::<AtkStateSet>(),
        },
    ),
    (
        "AtkStateSetClass",
        Layout {
            size: size_of::<AtkStateSetClass>(),
            alignment: align_of::<AtkStateSetClass>(),
        },
    ),
    (
        "AtkStateType",
        Layout {
            size: size_of::<AtkStateType>(),
            alignment: align_of::<AtkStateType>(),
        },
    ),
    (
        "AtkStreamableContentIface",
        Layout {
            size: size_of::<AtkStreamableContentIface>(),
            alignment: align_of::<AtkStreamableContentIface>(),
        },
    ),
    (
        "AtkTableCellIface",
        Layout {
            size: size_of::<AtkTableCellIface>(),
            alignment: align_of::<AtkTableCellIface>(),
        },
    ),
    (
        "AtkTableIface",
        Layout {
            size: size_of::<AtkTableIface>(),
            alignment: align_of::<AtkTableIface>(),
        },
    ),
    (
        "AtkTextAttribute",
        Layout {
            size: size_of::<AtkTextAttribute>(),
            alignment: align_of::<AtkTextAttribute>(),
        },
    ),
    (
        "AtkTextBoundary",
        Layout {
            size: size_of::<AtkTextBoundary>(),
            alignment: align_of::<AtkTextBoundary>(),
        },
    ),
    (
        "AtkTextClipType",
        Layout {
            size: size_of::<AtkTextClipType>(),
            alignment: align_of::<AtkTextClipType>(),
        },
    ),
    (
        "AtkTextGranularity",
        Layout {
            size: size_of::<AtkTextGranularity>(),
            alignment: align_of::<AtkTextGranularity>(),
        },
    ),
    (
        "AtkTextIface",
        Layout {
            size: size_of::<AtkTextIface>(),
            alignment: align_of::<AtkTextIface>(),
        },
    ),
    (
        "AtkTextRange",
        Layout {
            size: size_of::<AtkTextRange>(),
            alignment: align_of::<AtkTextRange>(),
        },
    ),
    (
        "AtkTextRectangle",
        Layout {
            size: size_of::<AtkTextRectangle>(),
            alignment: align_of::<AtkTextRectangle>(),
        },
    ),
    (
        "AtkUtil",
        Layout {
            size: size_of::<AtkUtil>(),
            alignment: align_of::<AtkUtil>(),
        },
    ),
    (
        "AtkUtilClass",
        Layout {
            size: size_of::<AtkUtilClass>(),
            alignment: align_of::<AtkUtilClass>(),
        },
    ),
    (
        "AtkValueIface",
        Layout {
            size: size_of::<AtkValueIface>(),
            alignment: align_of::<AtkValueIface>(),
        },
    ),
    (
        "AtkValueType",
        Layout {
            size: size_of::<AtkValueType>(),
            alignment: align_of::<AtkValueType>(),
        },
    ),
    (
        "AtkWindowIface",
        Layout {
            size: size_of::<AtkWindowIface>(),
            alignment: align_of::<AtkWindowIface>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(guint) ATK_HYPERLINK_IS_INLINE", "1"),
    ("(gint) ATK_KEY_EVENT_LAST_DEFINED", "2"),
    ("(gint) ATK_KEY_EVENT_PRESS", "0"),
    ("(gint) ATK_KEY_EVENT_RELEASE", "1"),
    ("(gint) ATK_LAYER_BACKGROUND", "1"),
    ("(gint) ATK_LAYER_CANVAS", "2"),
    ("(gint) ATK_LAYER_INVALID", "0"),
    ("(gint) ATK_LAYER_MDI", "4"),
    ("(gint) ATK_LAYER_OVERLAY", "6"),
    ("(gint) ATK_LAYER_POPUP", "5"),
    ("(gint) ATK_LAYER_WIDGET", "3"),
    ("(gint) ATK_LAYER_WINDOW", "7"),
    ("(gint) ATK_RELATION_CONTROLLED_BY", "1"),
    ("(gint) ATK_RELATION_CONTROLLER_FOR", "2"),
    ("(gint) ATK_RELATION_DESCRIBED_BY", "14"),
    ("(gint) ATK_RELATION_DESCRIPTION_FOR", "15"),
    ("(gint) ATK_RELATION_DETAILS", "17"),
    ("(gint) ATK_RELATION_DETAILS_FOR", "18"),
    ("(gint) ATK_RELATION_EMBEDDED_BY", "11"),
    ("(gint) ATK_RELATION_EMBEDS", "10"),
    ("(gint) ATK_RELATION_ERROR_FOR", "20"),
    ("(gint) ATK_RELATION_ERROR_MESSAGE", "19"),
    ("(gint) ATK_RELATION_FLOWS_FROM", "8"),
    ("(gint) ATK_RELATION_FLOWS_TO", "7"),
    ("(gint) ATK_RELATION_LABELLED_BY", "4"),
    ("(gint) ATK_RELATION_LABEL_FOR", "3"),
    ("(gint) ATK_RELATION_LAST_DEFINED", "21"),
    ("(gint) ATK_RELATION_MEMBER_OF", "5"),
    ("(gint) ATK_RELATION_NODE_CHILD_OF", "6"),
    ("(gint) ATK_RELATION_NODE_PARENT_OF", "16"),
    ("(gint) ATK_RELATION_NULL", "0"),
    ("(gint) ATK_RELATION_PARENT_WINDOW_OF", "13"),
    ("(gint) ATK_RELATION_POPUP_FOR", "12"),
    ("(gint) ATK_RELATION_SUBWINDOW_OF", "9"),
    ("(gint) ATK_ROLE_ACCEL_LABEL", "1"),
    ("(gint) ATK_ROLE_ALERT", "2"),
    ("(gint) ATK_ROLE_ANIMATION", "3"),
    ("(gint) ATK_ROLE_APPLICATION", "73"),
    ("(gint) ATK_ROLE_ARROW", "4"),
    ("(gint) ATK_ROLE_ARTICLE", "107"),
    ("(gint) ATK_ROLE_AUDIO", "104"),
    ("(gint) ATK_ROLE_AUTOCOMPLETE", "74"),
    ("(gint) ATK_ROLE_BLOCK_QUOTE", "103"),
    ("(gint) ATK_ROLE_CALENDAR", "5"),
    ("(gint) ATK_ROLE_CANVAS", "6"),
    ("(gint) ATK_ROLE_CAPTION", "79"),
    ("(gint) ATK_ROLE_CHART", "78"),
    ("(gint) ATK_ROLE_CHECK_BOX", "7"),
    ("(gint) ATK_ROLE_CHECK_MENU_ITEM", "8"),
    ("(gint) ATK_ROLE_COLOR_CHOOSER", "9"),
    ("(gint) ATK_ROLE_COLUMN_HEADER", "10"),
    ("(gint) ATK_ROLE_COMBO_BOX", "11"),
    ("(gint) ATK_ROLE_COMMENT", "95"),
    ("(gint) ATK_ROLE_CONTENT_DELETION", "123"),
    ("(gint) ATK_ROLE_CONTENT_INSERTION", "124"),
    ("(gint) ATK_ROLE_DATE_EDITOR", "12"),
    ("(gint) ATK_ROLE_DEFINITION", "106"),
    ("(gint) ATK_ROLE_DESCRIPTION_LIST", "114"),
    ("(gint) ATK_ROLE_DESCRIPTION_TERM", "115"),
    ("(gint) ATK_ROLE_DESCRIPTION_VALUE", "116"),
    ("(gint) ATK_ROLE_DESKTOP_FRAME", "14"),
    ("(gint) ATK_ROLE_DESKTOP_ICON", "13"),
    ("(gint) ATK_ROLE_DIAL", "15"),
    ("(gint) ATK_ROLE_DIALOG", "16"),
    ("(gint) ATK_ROLE_DIRECTORY_PANE", "17"),
    ("(gint) ATK_ROLE_DOCUMENT_EMAIL", "94"),
    ("(gint) ATK_ROLE_DOCUMENT_FRAME", "80"),
    ("(gint) ATK_ROLE_DOCUMENT_PRESENTATION", "91"),
    ("(gint) ATK_ROLE_DOCUMENT_SPREADSHEET", "90"),
    ("(gint) ATK_ROLE_DOCUMENT_TEXT", "92"),
    ("(gint) ATK_ROLE_DOCUMENT_WEB", "93"),
    ("(gint) ATK_ROLE_DRAWING_AREA", "18"),
    ("(gint) ATK_ROLE_EDITBAR", "75"),
    ("(gint) ATK_ROLE_EMBEDDED", "76"),
    ("(gint) ATK_ROLE_ENTRY", "77"),
    ("(gint) ATK_ROLE_FILE_CHOOSER", "19"),
    ("(gint) ATK_ROLE_FILLER", "20"),
    ("(gint) ATK_ROLE_FONT_CHOOSER", "21"),
    ("(gint) ATK_ROLE_FOOTER", "70"),
    ("(gint) ATK_ROLE_FOOTNOTE", "122"),
    ("(gint) ATK_ROLE_FORM", "85"),
    ("(gint) ATK_ROLE_FRAME", "22"),
    ("(gint) ATK_ROLE_GLASS_PANE", "23"),
    ("(gint) ATK_ROLE_GROUPING", "97"),
    ("(gint) ATK_ROLE_HEADER", "69"),
    ("(gint) ATK_ROLE_HEADING", "81"),
    ("(gint) ATK_ROLE_HTML_CONTAINER", "24"),
    ("(gint) ATK_ROLE_ICON", "25"),
    ("(gint) ATK_ROLE_IMAGE", "26"),
    ("(gint) ATK_ROLE_IMAGE_MAP", "98"),
    ("(gint) ATK_ROLE_INFO_BAR", "100"),
    ("(gint) ATK_ROLE_INPUT_METHOD_WINDOW", "87"),
    ("(gint) ATK_ROLE_INTERNAL_FRAME", "27"),
    ("(gint) ATK_ROLE_INVALID", "0"),
    ("(gint) ATK_ROLE_LABEL", "28"),
    ("(gint) ATK_ROLE_LANDMARK", "108"),
    ("(gint) ATK_ROLE_LAST_DEFINED", "127"),
    ("(gint) ATK_ROLE_LAYERED_PANE", "29"),
    ("(gint) ATK_ROLE_LEVEL_BAR", "101"),
    ("(gint) ATK_ROLE_LINK", "86"),
    ("(gint) ATK_ROLE_LIST", "30"),
    ("(gint) ATK_ROLE_LIST_BOX", "96"),
    ("(gint) ATK_ROLE_LIST_ITEM", "31"),
    ("(gint) ATK_ROLE_LOG", "109"),
    ("(gint) ATK_ROLE_MARK", "125"),
    ("(gint) ATK_ROLE_MARQUEE", "110"),
    ("(gint) ATK_ROLE_MATH", "111"),
    ("(gint) ATK_ROLE_MATH_FRACTION", "118"),
    ("(gint) ATK_ROLE_MATH_ROOT", "119"),
    ("(gint) ATK_ROLE_MENU", "32"),
    ("(gint) ATK_ROLE_MENU_BAR", "33"),
    ("(gint) ATK_ROLE_MENU_ITEM", "34"),
    ("(gint) ATK_ROLE_NOTIFICATION", "99"),
    ("(gint) ATK_ROLE_OPTION_PANE", "35"),
    ("(gint) ATK_ROLE_PAGE", "82"),
    ("(gint) ATK_ROLE_PAGE_TAB", "36"),
    ("(gint) ATK_ROLE_PAGE_TAB_LIST", "37"),
    ("(gint) ATK_ROLE_PANEL", "38"),
    ("(gint) ATK_ROLE_PARAGRAPH", "71"),
    ("(gint) ATK_ROLE_PASSWORD_TEXT", "39"),
    ("(gint) ATK_ROLE_POPUP_MENU", "40"),
    ("(gint) ATK_ROLE_PROGRESS_BAR", "41"),
    ("(gint) ATK_ROLE_PUSH_BUTTON", "42"),
    ("(gint) ATK_ROLE_RADIO_BUTTON", "43"),
    ("(gint) ATK_ROLE_RADIO_MENU_ITEM", "44"),
    ("(gint) ATK_ROLE_RATING", "112"),
    ("(gint) ATK_ROLE_REDUNDANT_OBJECT", "84"),
    ("(gint) ATK_ROLE_ROOT_PANE", "45"),
    ("(gint) ATK_ROLE_ROW_HEADER", "46"),
    ("(gint) ATK_ROLE_RULER", "72"),
    ("(gint) ATK_ROLE_SCROLL_BAR", "47"),
    ("(gint) ATK_ROLE_SCROLL_PANE", "48"),
    ("(gint) ATK_ROLE_SECTION", "83"),
    ("(gint) ATK_ROLE_SEPARATOR", "49"),
    ("(gint) ATK_ROLE_SLIDER", "50"),
    ("(gint) ATK_ROLE_SPIN_BUTTON", "52"),
    ("(gint) ATK_ROLE_SPLIT_PANE", "51"),
    ("(gint) ATK_ROLE_STATIC", "117"),
    ("(gint) ATK_ROLE_STATUSBAR", "53"),
    ("(gint) ATK_ROLE_SUBSCRIPT", "120"),
    ("(gint) ATK_ROLE_SUGGESTION", "126"),
    ("(gint) ATK_ROLE_SUPERSCRIPT", "121"),
    ("(gint) ATK_ROLE_TABLE", "54"),
    ("(gint) ATK_ROLE_TABLE_CELL", "55"),
    ("(gint) ATK_ROLE_TABLE_COLUMN_HEADER", "56"),
    ("(gint) ATK_ROLE_TABLE_ROW", "88"),
    ("(gint) ATK_ROLE_TABLE_ROW_HEADER", "57"),
    ("(gint) ATK_ROLE_TEAR_OFF_MENU_ITEM", "58"),
    ("(gint) ATK_ROLE_TERMINAL", "59"),
    ("(gint) ATK_ROLE_TEXT", "60"),
    ("(gint) ATK_ROLE_TIMER", "113"),
    ("(gint) ATK_ROLE_TITLE_BAR", "102"),
    ("(gint) ATK_ROLE_TOGGLE_BUTTON", "61"),
    ("(gint) ATK_ROLE_TOOL_BAR", "62"),
    ("(gint) ATK_ROLE_TOOL_TIP", "63"),
    ("(gint) ATK_ROLE_TREE", "64"),
    ("(gint) ATK_ROLE_TREE_ITEM", "89"),
    ("(gint) ATK_ROLE_TREE_TABLE", "65"),
    ("(gint) ATK_ROLE_UNKNOWN", "66"),
    ("(gint) ATK_ROLE_VIDEO", "105"),
    ("(gint) ATK_ROLE_VIEWPORT", "67"),
    ("(gint) ATK_ROLE_WINDOW", "68"),
    ("(gint) ATK_SCROLL_ANYWHERE", "6"),
    ("(gint) ATK_SCROLL_BOTTOM_EDGE", "3"),
    ("(gint) ATK_SCROLL_BOTTOM_RIGHT", "1"),
    ("(gint) ATK_SCROLL_LEFT_EDGE", "4"),
    ("(gint) ATK_SCROLL_RIGHT_EDGE", "5"),
    ("(gint) ATK_SCROLL_TOP_EDGE", "2"),
    ("(gint) ATK_SCROLL_TOP_LEFT", "0"),
    ("(gint) ATK_STATE_ACTIVE", "1"),
    ("(gint) ATK_STATE_ANIMATED", "37"),
    ("(gint) ATK_STATE_ARMED", "2"),
    ("(gint) ATK_STATE_BUSY", "3"),
    ("(gint) ATK_STATE_CHECKABLE", "39"),
    ("(gint) ATK_STATE_CHECKED", "4"),
    ("(gint) ATK_STATE_DEFAULT", "36"),
    ("(gint) ATK_STATE_DEFUNCT", "5"),
    ("(gint) ATK_STATE_EDITABLE", "6"),
    ("(gint) ATK_STATE_ENABLED", "7"),
    ("(gint) ATK_STATE_EXPANDABLE", "8"),
    ("(gint) ATK_STATE_EXPANDED", "9"),
    ("(gint) ATK_STATE_FOCUSABLE", "10"),
    ("(gint) ATK_STATE_FOCUSED", "11"),
    ("(gint) ATK_STATE_HAS_POPUP", "40"),
    ("(gint) ATK_STATE_HAS_TOOLTIP", "41"),
    ("(gint) ATK_STATE_HORIZONTAL", "12"),
    ("(gint) ATK_STATE_ICONIFIED", "13"),
    ("(gint) ATK_STATE_INDETERMINATE", "30"),
    ("(gint) ATK_STATE_INVALID", "0"),
    ("(gint) ATK_STATE_INVALID_ENTRY", "33"),
    ("(gint) ATK_STATE_LAST_DEFINED", "43"),
    ("(gint) ATK_STATE_MANAGES_DESCENDANTS", "29"),
    ("(gint) ATK_STATE_MODAL", "14"),
    ("(gint) ATK_STATE_MULTISELECTABLE", "16"),
    ("(gint) ATK_STATE_MULTI_LINE", "15"),
    ("(gint) ATK_STATE_OPAQUE", "17"),
    ("(gint) ATK_STATE_PRESSED", "18"),
    ("(gint) ATK_STATE_READ_ONLY", "42"),
    ("(gint) ATK_STATE_REQUIRED", "32"),
    ("(gint) ATK_STATE_RESIZABLE", "19"),
    ("(gint) ATK_STATE_SELECTABLE", "20"),
    ("(gint) ATK_STATE_SELECTABLE_TEXT", "35"),
    ("(gint) ATK_STATE_SELECTED", "21"),
    ("(gint) ATK_STATE_SENSITIVE", "22"),
    ("(gint) ATK_STATE_SHOWING", "23"),
    ("(gint) ATK_STATE_SINGLE_LINE", "24"),
    ("(gint) ATK_STATE_STALE", "25"),
    ("(gint) ATK_STATE_SUPPORTS_AUTOCOMPLETION", "34"),
    ("(gint) ATK_STATE_TRANSIENT", "26"),
    ("(gint) ATK_STATE_TRUNCATED", "31"),
    ("(gint) ATK_STATE_VERTICAL", "27"),
    ("(gint) ATK_STATE_VISIBLE", "28"),
    ("(gint) ATK_STATE_VISITED", "38"),
    ("(gint) ATK_TEXT_ATTR_BG_COLOR", "18"),
    ("(gint) ATK_TEXT_ATTR_BG_FULL_HEIGHT", "9"),
    ("(gint) ATK_TEXT_ATTR_BG_STIPPLE", "20"),
    ("(gint) ATK_TEXT_ATTR_DIRECTION", "23"),
    ("(gint) ATK_TEXT_ATTR_EDITABLE", "5"),
    ("(gint) ATK_TEXT_ATTR_FAMILY_NAME", "17"),
    ("(gint) ATK_TEXT_ATTR_FG_COLOR", "19"),
    ("(gint) ATK_TEXT_ATTR_FG_STIPPLE", "21"),
    ("(gint) ATK_TEXT_ATTR_INDENT", "3"),
    ("(gint) ATK_TEXT_ATTR_INVALID", "0"),
    ("(gint) ATK_TEXT_ATTR_INVISIBLE", "4"),
    ("(gint) ATK_TEXT_ATTR_JUSTIFICATION", "24"),
    ("(gint) ATK_TEXT_ATTR_LANGUAGE", "16"),
    ("(gint) ATK_TEXT_ATTR_LAST_DEFINED", "29"),
    ("(gint) ATK_TEXT_ATTR_LEFT_MARGIN", "1"),
    ("(gint) ATK_TEXT_ATTR_PIXELS_ABOVE_LINES", "6"),
    ("(gint) ATK_TEXT_ATTR_PIXELS_BELOW_LINES", "7"),
    ("(gint) ATK_TEXT_ATTR_PIXELS_INSIDE_WRAP", "8"),
    ("(gint) ATK_TEXT_ATTR_RIGHT_MARGIN", "2"),
    ("(gint) ATK_TEXT_ATTR_RISE", "10"),
    ("(gint) ATK_TEXT_ATTR_SCALE", "14"),
    ("(gint) ATK_TEXT_ATTR_SIZE", "13"),
    ("(gint) ATK_TEXT_ATTR_STRETCH", "25"),
    ("(gint) ATK_TEXT_ATTR_STRIKETHROUGH", "12"),
    ("(gint) ATK_TEXT_ATTR_STYLE", "27"),
    ("(gint) ATK_TEXT_ATTR_TEXT_POSITION", "28"),
    ("(gint) ATK_TEXT_ATTR_UNDERLINE", "11"),
    ("(gint) ATK_TEXT_ATTR_VARIANT", "26"),
    ("(gint) ATK_TEXT_ATTR_WEIGHT", "15"),
    ("(gint) ATK_TEXT_ATTR_WRAP_MODE", "22"),
    ("(gint) ATK_TEXT_BOUNDARY_CHAR", "0"),
    ("(gint) ATK_TEXT_BOUNDARY_LINE_END", "6"),
    ("(gint) ATK_TEXT_BOUNDARY_LINE_START", "5"),
    ("(gint) ATK_TEXT_BOUNDARY_SENTENCE_END", "4"),
    ("(gint) ATK_TEXT_BOUNDARY_SENTENCE_START", "3"),
    ("(gint) ATK_TEXT_BOUNDARY_WORD_END", "2"),
    ("(gint) ATK_TEXT_BOUNDARY_WORD_START", "1"),
    ("(gint) ATK_TEXT_CLIP_BOTH", "3"),
    ("(gint) ATK_TEXT_CLIP_MAX", "2"),
    ("(gint) ATK_TEXT_CLIP_MIN", "1"),
    ("(gint) ATK_TEXT_CLIP_NONE", "0"),
    ("(gint) ATK_TEXT_GRANULARITY_CHAR", "0"),
    ("(gint) ATK_TEXT_GRANULARITY_LINE", "3"),
    ("(gint) ATK_TEXT_GRANULARITY_PARAGRAPH", "4"),
    ("(gint) ATK_TEXT_GRANULARITY_SENTENCE", "2"),
    ("(gint) ATK_TEXT_GRANULARITY_WORD", "1"),
    ("(gint) ATK_VALUE_ACCEPTABLE", "2"),
    ("(gint) ATK_VALUE_BAD", "11"),
    ("(gint) ATK_VALUE_BEST", "14"),
    ("(gint) ATK_VALUE_GOOD", "12"),
    ("(gint) ATK_VALUE_HIGH", "8"),
    ("(gint) ATK_VALUE_LAST_DEFINED", "15"),
    ("(gint) ATK_VALUE_LOW", "6"),
    ("(gint) ATK_VALUE_MEDIUM", "7"),
    ("(gint) ATK_VALUE_STRONG", "3"),
    ("(gint) ATK_VALUE_VERY_BAD", "10"),
    ("(gint) ATK_VALUE_VERY_GOOD", "13"),
    ("(gint) ATK_VALUE_VERY_HIGH", "9"),
    ("(gint) ATK_VALUE_VERY_LOW", "5"),
    ("(gint) ATK_VALUE_VERY_STRONG", "4"),
    ("(gint) ATK_VALUE_VERY_WEAK", "0"),
    ("(gint) ATK_VALUE_WEAK", "1"),
    ("(gint) ATK_XY_PARENT", "2"),
    ("(gint) ATK_XY_SCREEN", "0"),
    ("(gint) ATK_XY_WINDOW", "1"),
];