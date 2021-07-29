use super::*;

pub struct Ext {}

impl Ext {
    pub fn float_input(
        ui: &Ui,
        handle: &Handle,
        label: &str,    /* = ""*/
        align: Align,   /* = Left*/
        precision: f32, /* = 1000.0*/
    ) -> f32 {
        // handle.text = Std.string(Math.round(handle.value * precision) / precision);
        // let text = ui.textInput(handle, label, align);
        // handle.value = Std.parseFloat(text);
        // return handle.value;
        unimplemented!()
    }

    fn init_path(handle: &Handle, system_id: String) {
        // handle.text = systemId == "Windows" ? "C:\\Users" : "/";
        // // %HOMEDRIVE% + %HomePath%
        // // ~
        unimplemented!()
    }

    // pub static let dataPath = "";
    // static let lastPath = "";

    pub fn file_browser(ui: &Ui, handle: &Handle, folders_only: bool /*= false*/) -> String {
        // let sep = "/";

        // #if kha_krom

        // let cmd = "ls ";
        // let systemId = System.systemId;
        // if (systemId == "Windows") {
        // 	cmd = "dir /b ";
        // 	if (foldersOnly) cmd += "/ad ";
        // 	sep = "\\";
        // 	handle.text = StringTools.replace(handle.text, "\\\\", "\\");
        // 	handle.text = StringTools.replace(handle.text, "\r", "");
        // }
        // if (handle.text == "") initPath(handle, systemId);

        // let save = Krom.getFilesLocation() + sep + dataPath + "dir.txt";
        // if (handle.text != lastPath) Krom.sysCommand(cmd + '"' + handle.text + '"' + " > " + '"' + save + '"');
        // lastPath = handle.text;
        // let str = haxe.io.Bytes.ofData(Krom.loadBlob(save)).toString();
        // let files = str.split("\n");

        // #elseif kha_kore

        // if (handle.text == "") initPath(handle, System.systemId);
        // let files = sys.FileSystem.isDirectory(handle.text) ? sys.FileSystem.readDirectory(handle.text) : [];

        // #elseif kha_webgl

        // let files: Vec<String> = [];

        // let userAgent = untyped navigator.userAgent.toLowerCase();
        // if (userAgent.indexOf(" electron/") > -1) {
        // 	if (handle.text == "") {
        // 		let pp = untyped window.process.platform;
        // 		let systemId = pp == "win32" ? "Windows" : (pp == "darwin" ? "OSX" : "Linux");
        // 		initPath(handle, systemId);
        // 	}
        // 	try {
        // 		files = untyped require("fs").readdirSync(handle.text);
        // 	}
        // 	catch (e: Dynamic) {
        // 		// Non-directory item selected
        // 	}
        // }

        // #else

        // let files: Vec<String> = [];

        // #end

        // // Up directory
        // let i1 = handle.text.indexOf("/");
        // let i2 = handle.text.indexOf("\\");
        // let nested =
        // 	(i1 > -1 && handle.text.length - 1 > i1) ||
        // 	(i2 > -1 && handle.text.length - 1 > i2);
        // handle.changed = false;
        // if (nested && ui.button("..", Align.Left)) {
        // 	handle.changed = ui.changed = true;
        // 	handle.text = handle.text.substring(0, handle.text.lastIndexOf(sep));
        // 	// Drive root
        // 	if (handle.text.length == 2 && handle.text.charAt(1) == ":") handle.text += sep;
        // }

        // // Directory contents
        // for (f in files) {
        // 	if (f == "" || f.charAt(0) == ".") continue; // Skip hidden
        // 	if (ui.button(f, Align.Left)) {
        // 		handle.changed = ui.changed = true;
        // 		if (handle.text.charAt(handle.text.length - 1) != sep) handle.text += sep;
        // 		handle.text += f;
        // 	}
        // }

        // return handle.text;
        unimplemented!()
    }

    pub fn inline_radio(
        ui: &Ui,
        handle: &Handle,
        texts: &[&str],
        align: Align, /*= Left*/
    ) -> i32 {
        // if (!ui.isVisible(ui.ELEMENT_H())) {
        // 	ui.endElement();
        // 	return handle.position;
        // }
        // let step = ui._w / texts.length;
        // let hovered = -1;
        // if (ui.getHover()) {
        // 	let ix = Std.int(ui.inputX - ui._x - ui._windowX);
        // 	for (i in 0...texts.length) {
        // 		if (ix < i * step + step) {
        // 			hovered = i;
        // 			break;
        // 		}
        // 	}
        // }

        // if (ui.getReleased()) {
        // 	handle.position = hovered;
        // 	handle.changed = ui.changed = true;
        // } else {
        // 	handle.changed = false;
        // }

        // for (i in 0...texts.length) {
        // 	if (handle.position == i) {
        // 		ui.g.color = ui.t.ACCENT_HOVER_COL;
        // 		if (!ui.enabled) ui.fadeColor();
        // 		ui.g.fillRect(ui._x + step * i, ui._y + ui.buttonOffsetY, step, ui.BUTTON_H());
        // 	} else if (hovered == i) {
        // 		ui.g.color = ui.t.ACCENT_COL;
        // 		if (!ui.enabled) ui.fadeColor();
        // 		ui.g.drawRect(ui._x + step * i, ui._y + ui.buttonOffsetY, step, ui.BUTTON_H());
        // 	}
        // 	ui.g.color = ui.t.TEXT_COL; // Text
        // 	ui._x += step * i;
        // 	let _w = ui._w;
        // 	ui._w = Std.int(step);
        // 	ui.drawString(ui.g, texts[i], null, 0, align);
        // 	ui._x -= step * i;
        // 	ui._w = _w;
        // }
        // ui.endElement();
        // return handle.position;
        unimplemented!()
    }

    // static let wheelSelectedHande: Handle = null;

    pub fn color_wheel(
        ui: &Ui,
        handle: &Handle,
        alpha: bool,         /* = false*/
        w: Option<f32>,      /* = null*/
        color_preview: bool, /* = true*/
    ) -> Color {
        // if (w == null) {
        // 	w = ui._w;
        // }
        // rgbToHsv(handle.color.R, handle.color.G, handle.color.B, ar);
        // let chue = ar[0];
        // let csat = ar[1];
        // let cval = ar[2];
        // let calpha = handle.color.A;
        // // Wheel
        // let px = ui._x;
        // let py = ui._y;
        // let scroll = ui.currentWindow != null ? ui.currentWindow.scrollEnabled : false;
        // if (!scroll) {
        // 	w -= ui.SCROLL_W();
        // 	px += ui.SCROLL_W() / 2;
        // }
        // ui.image(ui.ops.color_wheel, Color.fromFloats(cval, cval, cval));
        // // Picker
        // let ph = ui._y - py;
        // let ox = px + w / 2;
        // let oy = py + ph / 2;
        // let cw = w * 0.7;
        // let cwh = cw / 2;
        // let cx = ox;
        // let cy = oy + csat * cwh; // Sat is distance from center
        // // Rotate around origin by hue
        // let theta = chue * (Math.PI * 2.0);
        // let cx2 = Math.cos(theta) * (cx - ox) - Math.sin(theta) * (cy - oy) + ox;
        // let cy2 = Math.sin(theta) * (cx - ox) + Math.cos(theta) * (cy - oy) + oy;
        // cx = cx2;
        // cy = cy2;

        // ui.g.color = 0xff000000;
        // ui.g.fillRect(cx - 3 * ui.SCALE(), cy - 3 * ui.SCALE(), 6 * ui.SCALE(), 6 * ui.SCALE());
        // ui.g.color = 0xffffffff;
        // ui.g.fillRect(cx - 2 * ui.SCALE(), cy - 2 * ui.SCALE(), 4 * ui.SCALE(), 4 * ui.SCALE());

        // if (alpha) {
        // 	let alphaHandle = handle.nest(1, {value: Math.round(calpha * 100) / 100});
        // 	calpha = ui.slider(alphaHandle, "Alpha", 0.0, 1.0, true);
        // 	if (alphaHandle.changed) handle.changed = ui.changed = true;
        // }
        // // Mouse picking
        // let gx = ox + ui._windowX;
        // let gy = oy + ui._windowY;
        // if (ui.inputStarted && ui.getInputInRect(gx - cwh, gy - cwh, cw, cw)) wheelSelectedHande = handle;
        // if (ui.inputReleased) wheelSelectedHande = null;
        // if (ui.inputDown && wheelSelectedHande == handle) {
        // 	csat = Math.min(dist(gx, gy, ui.inputX, ui.inputY), cwh) / cwh;
        // 	let angle = Math.atan2(ui.inputX - gx, ui.inputY - gy);
        // 	if (angle < 0) angle = Math.PI + (Math.PI - Math.abs(angle));
        // 	angle = Math.PI * 2 - angle;
        // 	chue = angle / (Math.PI * 2);
        // 	handle.changed = ui.changed = true;
        // }
        // // Save as rgb
        // hsvToRgb(chue, csat, cval, ar);
        // handle.color = Color.fromFloats(ar[0], ar[1], ar[2], calpha);

        // if (colorPreview) ui.text("", Right, handle.color);

        // let pos = Ext.inlineRadio(ui, Id.handle(), ["RGB", "HSV", "Hex"]);
        // let h0 = handle.nest(0).nest(0);
        // let h1 = handle.nest(0).nest(1);
        // let h2 = handle.nest(0).nest(2);
        // if (pos == 0) {
        // 	h0.value = handle.color.R;

        // 	handle.color.R = ui.slider(h0, "R", 0, 1, true);
        // 	h1.value = handle.color.G;

        // 	handle.color.G = ui.slider(h1, "G", 0, 1, true);
        // 	h2.value = handle.color.B;
        // 	handle.color.B = ui.slider(h2, "B", 0, 1, true);
        // } else if (pos == 1) {
        // 	rgbToHsv(handle.color.R, handle.color.G, handle.color.B, ar);
        // 	h0.value = ar[0];
        // 	h1.value = ar[1];
        // 	h2.value = ar[2];
        // 	let chue = ui.slider(h0, "H", 0, 1, true);
        // 	let csat = ui.slider(h1, "S", 0, 1, true);
        // 	let cval = ui.slider(h2, "V", 0, 1, true);
        // 	hsvToRgb(chue, csat, cval, ar);
        // 	handle.color = Color.fromFloats(ar[0], ar[1], ar[2]);
        // } else if (pos == 2) {
        // 	#if js
        // 	handle.text = untyped (handle.color >>> 0).toString(16);
        // 	handle.color = untyped parseInt(ui.textInput(handle, "#"), 16);
        // 	#end
        // }
        // if (h0.changed || h1.changed || h2.changed) {
        // 	handle.changed = ui.changed = true;
        // }
        // return handle.color;
        unimplemented!()
    }

    pub fn text_area(
        ui: Ui,
        handle: &Handle,
        align: Align,   /* = Align.Left*/
        editable: bool, /*= true*/
    ) -> String {
        // handle.text = StringTools.replace(handle.text, "\t", "    ");
        // let lines = handle.text.split("\n");
        // let selected = ui.textSelectedHandle == handle; // Text being edited
        // let cursorStartX = ui.cursorX;
        // let keyPressed = selected && ui.isKeyPressed;
        // ui.highlightOnSelect = false;
        // ui.tabSwitchEnabled = false;
        // ui.g.color = ui.t.SEPARATOR_COL;
        // ui.drawRect(ui.g, true, ui._x + ui.buttonOffsetY, ui._y + ui.buttonOffsetY, ui._w - ui.buttonOffsetY * 2, lines.length * ui.ELEMENT_H() - ui.buttonOffsetY * 2);

        // for (i in 0...lines.length) { // Draw lines
        // 	if ((!selected && ui.getHover()) || (selected && i == handle.position)) {
        // 		handle.position = i; // Set active line
        // 		handle.text = lines[i];
        // 		ui.textInput(handle, "", align, editable);
        // 		if (keyPressed && ui.key != KeyCode.Return && ui.key != KeyCode.Escape) { // Edit text
        // 			lines[i] = ui.textSelected;
        // 		}
        // 	}
        // 	else {
        // 		ui.text(lines[i], align);
        // 	}
        // 	ui._y -= ui.ELEMENT_OFFSET();
        // }
        // ui._y += ui.ELEMENT_OFFSET();

        // if (keyPressed) {
        // 	// Move cursor vertically
        // 	if (ui.key == KeyCode.Down && handle.position < lines.length - 1) {
        // 		handle.position++;
        // 	}
        // 	if (ui.key == KeyCode.Up && handle.position > 0) {
        // 		handle.position--;
        // 	}
        // 	// New line
        // 	if (editable && ui.key == KeyCode.Return) {
        // 		handle.position++;
        // 		lines.insert(handle.position, lines[handle.position - 1].substr(ui.cursorX));
        // 		lines[handle.position - 1] = lines[handle.position - 1].substr(0, ui.cursorX);
        // 		ui.startTextEdit(handle);
        // 		ui.cursorX = ui.highlightAnchor = 0;
        // 	}
        // 	// Delete line
        // 	if (editable && ui.key == KeyCode.Backspace && cursorStartX == 0 && handle.position > 0) {
        // 		handle.position--;
        // 		ui.cursorX = ui.highlightAnchor = lines[handle.position].length;
        // 		lines[handle.position] += lines[handle.position + 1];
        // 		lines.splice(handle.position + 1, 1);
        // 	}
        // 	ui.textSelected = lines[handle.position];
        // }

        // ui.highlightOnSelect = true;
        // ui.tabSwitchEnabled = true;
        // handle.text = lines.join("\n");
        // return handle.text;
        unimplemented!()
    }

    // let _ELEMENT_OFFSET = 0;
    // let _BUTTON_COL = 0;

    pub fn begin_menu(ui: Ui) {
        // _ELEMENT_OFFSET = ui.t.ELEMENT_OFFSET;
        // _BUTTON_COL = ui.t.BUTTON_COL;
        // ui.t.ELEMENT_OFFSET = 0;
        // ui.t.BUTTON_COL = ui.t.SEPARATOR_COL;
        // ui.g.color = ui.t.SEPARATOR_COL;
        // ui.g.fillRect(0, 0, ui._windowW, MENUBAR_H(ui));
        unimplemented!()
    }

    pub fn end_menu(ui: Ui) {
        // ui.t.ELEMENT_OFFSET = _ELEMENT_OFFSET;
        // ui.t.BUTTON_COL = _BUTTON_COL;
        unimplemented!()
    }

    pub fn menu_button(ui: Ui, text: String) -> bool {
        // ui._w = Std.int(ui.ops.font.width(ui.fontSize, text) + 25 * ui.SCALE());
        // return ui.button(text);
        unimplemented!()
    }

    pub fn menubar_h(ui: Ui) -> f32 {
        // return ui.BUTTON_H() * 1.1 + 2 + ui.buttonOffsetY;
        unimplemented!()
    }

    fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        // let vx = x1 - x2;
        // let vy = y1 - y2;
        // return Math.sqrt(vx * vx + vy * vy);
        unimplemented!()
    }

    fn fract(f: f32) -> f32 {
        // return f - Std.int(f);
        unimplemented!()
    }

    fn mix(x: f32, y: f32, a: f32) -> f32 {
        // return x * (1.0 - a) + y * a;
        unimplemented!()
    }

    fn clamp(x: f32, min_val: f32, max_val: f32) -> f32 {
        // return Math.min(Math.max(x, minVal), maxVal);
        unimplemented!()
    }

    fn step(edge: f32, x: f32) -> f32 {
        // return x < edge ? 0.0 : 1.0;
        unimplemented!()
    }

    const kx: f32 = 1.0;
    const ky: f32 = 2.0 / 3.0;
    const kz: f32 = 1.0 / 3.0;
    const kw: f32 = 3.0;
    // static ar = [0.0, 0.0, 0.0];

    fn hsv_to_rgb(cr: f32, cg: f32, cb: f32, out: Vec<f32>) {
        // let px = Math.abs(fract(cR + kx) * 6.0 - kw);
        // let py = Math.abs(fract(cR + ky) * 6.0 - kw);
        // let pz = Math.abs(fract(cR + kz) * 6.0 - kw);
        // out[0] = cB * mix(kx, clamp(px - kx, 0.0, 1.0), cG);
        // out[1] = cB * mix(kx, clamp(py - kx, 0.0, 1.0), cG);
        // out[2] = cB * mix(kx, clamp(pz - kx, 0.0, 1.0), cG);
        unimplemented!()
    }

    const Kx: f32 = 0.0;
    const Ky: f32 = -1.0 / 3.0;
    const Kz: f32 = 2.0 / 3.0;
    const Kw: f32 = -1.0;
    const e: f32 = 1.0e-10;

    fn rgb_to_hsv(cr: f32, cg: f32, cb: f32, out: Vec<f32>) {
        // let px = mix(cB, cG, step(cB, cG));
        // let py = mix(cG, cB, step(cB, cG));
        // let pz = mix(Kw, Kx, step(cB, cG));
        // let pw = mix(Kz, Ky, step(cB, cG));
        // let qx = mix(px, cR, step(px, cR));
        // let qy = mix(py, py, step(px, cR));
        // let qz = mix(pw, pz, step(px, cR));
        // let qw = mix(cR, px, step(px, cR));
        // let d = qx - Math.min(qw, qy);
        // out[0] = Math.abs(qz + (qw - qy) / (6.0 * d + e));
        // out[1] = d / (qx + e);
        // out[2] = qx;
        unimplemented!()
    }
}
