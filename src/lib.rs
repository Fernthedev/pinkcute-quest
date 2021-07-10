#![feature(backtrace)]

use quest_hook::hook;
use quest_hook::libil2cpp::{Il2CppObject, Il2CppString};
use quest_hook::tracing::info;

#[no_mangle]
pub extern "C" fn setup() {
    quest_hook::setup!();
}

#[hook("TMPro", "TextMeshPro", "GenerateTextMesh")]
fn TextMeshPro_GenerateTextMesh(this: &mut Il2CppObject) {
    this.invoke_void("set_text", Il2CppString::new("Pink Cute"))
        .unwrap();
    this.invoke_void("ParseInputText", ()).unwrap();

    TextMeshPro_GenerateTextMesh.original(this);
}

#[hook("TMPro", "TextMeshProUGUI", "GenerateTextMesh")]
fn TextMeshProUGUI_GenerateTextMesh(this: &mut Il2CppObject) {
    this.invoke_void("set_text", Il2CppString::new("Pink Cute"))
        .unwrap();

    this.invoke_void("ParseInputText", ()).unwrap();

    TextMeshProUGUI_GenerateTextMesh.original(this);
}

#[hook("TMPro", "TextMeshPro", "SetText")]
fn TextMeshProUGUI_SetText(
    this: &mut Il2CppObject,
    input: Option<&mut Il2CppString>,
    syncTextInputBox: bool,
) {
    TextMeshProUGUI_SetText.original(this, Some(Il2CppString::new("Pink Cute")), syncTextInputBox);
}

#[no_mangle]
pub extern "C" fn load() {
    info!("Installing Pink Cute hooks!");

    TextMeshProUGUI_GenerateTextMesh.install();
    TextMeshPro_GenerateTextMesh.install();

    info!("Installed Pink Cute hooks!");
}
