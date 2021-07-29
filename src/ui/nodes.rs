#![allow(unused_variables)]

use super::*;

// using graphics2.GraphicsExtension;

// @:access(zui.Zui)
pub struct Nodes {
    // pub nodesDrag = false;
// pub nodesSelected: Vec<TNode> = [];
// pub panX = 0.0;
// pub panY = 0.0;
// pub zoom = 1.0;
// pub uiw = 0;
// pub uih = 0;
// pub _inputStarted = false;
// scaleFactor = 1.0;
// ELEMENT_H = 25;
// dragged = false;
// moveOnTop: TNode = null;
// linkDrag: TNodeLink = null;
// isNewLink = false;
// snapFromId = -1;
// snapToId = -1;
// snapSocket = 0;
// snapX = 0.0;
// snapY = 0.0;
// handle = new Zui.Handle();
// static elementsBaked = false;
// static socketImage: Image = null;
// static socketReleased = false;
// static clipboard = "";
// static boxSelect = false;
// static boxSelectX = 0;
// static boxSelectY = 0;
// static inline maxButtons = 9;

// pub static excludeRemove: Vec<String> = []; // No removal for listed node types
// pub static onLinkDrag: TNodeLink->Bool->Void = null;
// pub static onHeaderReleased: TNode->Void = null;
// pub static onSocketReleased: TNodeSocket->Void = null;
// pub static onCanvasReleased: Void->Void = null;
// pub static onNodeRemove: TNode->Void = null;
// pub static onCanvasControl: Void->CanvasControl = null; // Pan, zoom
}

impl Nodes {
    // #if zui_translate
    // pub static dynamic fn tr(id: String, vars: Map<String, String> = null) {
    // 	return id;
    // }
    // #else
    // pub static inline fn tr(id: String, vars: Map<String, String> = null) {
    // 	return id;
    // }
    // #end

    pub fn new() {}

    #[inline]
    pub fn scale() -> f32 {
        // return scaleFactor * zoom;
        unimplemented!()
    }

    #[inline]
    pub fn pan_x() -> f32 {
        // let zoomPan = (1.0 - zoom) * uiw / 2.5;
        // return panX * SCALE() + zoomPan;
        unimplemented!()
    }

    #[inline]
    pub fn pan_y() -> f32 {
        // let zoomPan = (1.0 - zoom) * uih / 2.5;
        // return panY * SCALE() + zoomPan;
        unimplemented!()
    }

    #[inline]
    pub fn line_h() -> i32 {
        // return Std.int(ELEMENT_H * SCALE());
        unimplemented!()
    }

    fn buttons_h(node: TNode) -> i32 {
        // let h = 0.0;
        // for (but in node.buttons) {
        // 	if (but.type == "RGBA") h += 235 * SCALE();
        // 	else if (but.type == "VECTOR") h += LINE_H() * 4;
        // 	else if (but.type == "CUSTOM") h += LINE_H() * but.height;
        // 	else h += LINE_H();
        // }
        // return Std.int(h);
        unimplemented!()
    }

    fn outputs_h(sockets: Vec<TNodeSocket>, length: Option<i32> /* = null*/) -> i32 {
        // let h = 0.0;
        // for (i in 0...(length == null ? sockets.length : length)) {
        // 	h += LINE_H();
        // }
        // return Std.int(h);
        unimplemented!()
    }

    fn inputs_h(
        canvas: TNodeCanvas,
        sockets: Vec<TNodeSocket>,
        length: Option<i32>, /* = null*/
    ) -> i32 {
        // let h = 0.0;
        // for (i in 0...(length == null ? sockets.length : length)) {
        // 	if (sockets[i].type == "VECTOR" && sockets[i].display == 1 && !inputLinked(canvas, sockets[i].node_id, i)) h += LINE_H() * 4;
        // 	else h += LINE_H();
        // }
        // return Std.int(h);
        unimplemented!()
    }

    #[inline]
    fn node_h(canvas: TNodeCanvas, node: TNode) -> i32 {
        // return Std.int(LINE_H() * 1.2 + INPUTS_H(canvas, node.inputs) + OUTPUTS_H(node.outputs) + BUTTONS_H(node));
        unimplemented!()
    }

    #[inline]
    fn node_w(node: TNode) -> i32 {
        // return Std.int((node.width != null ? node.width : 140) * SCALE());
        unimplemented!()
    }

    #[inline]
    fn node_x(node: TNode) -> f32 {
        // return node.x * SCALE() + PAN_X();
        unimplemented!()
    }

    #[inline]
    fn node_y(node: TNode) -> f32 {
        // return node.y * SCALE() + PAN_Y();
        unimplemented!()
    }

    #[inline]
    fn input_y(canvas: TNodeCanvas, sockets: Vec<TNodeSocket>, pos: i32) -> i32 {
        // return Std.int(LINE_H() * 1.62) + INPUTS_H(canvas, sockets, pos);
        unimplemented!()
    }

    #[inline]
    fn output_y(sockets: Vec<TNodeSocket>, pos: i32) -> i32 {
        // return Std.int(LINE_H() * 1.62) + OUTPUTS_H(sockets, pos);
        unimplemented!()
    }

    #[inline]
    pub fn p(f: f32) -> f32 {
        // return f * SCALE();
        unimplemented!()
    }

    pub fn get_node(&self, nodes: Vec<TNode>, id: i32) -> TNode {
        // for (node in nodes) if (node.id == id) return node;
        // return null;
        unimplemented!()
    }

    // let nodeId = -1;

    pub fn get_node_id(&self, nodes: Vec<TNode>) -> i32 {
        // if (nodeId == -1) for (n in nodes) if (nodeId < n.id) nodeId = n.id;
        // return ++nodeId;
        unimplemented!()
    }

    pub fn get_link_id(&self, links: Vec<TNodeLink>) -> i32 {
        // let id = 0;
        // for (l in links) if (l.id >= id) id = l.id + 1;
        // return id;
        unimplemented!()
    }

    pub fn get_socket_id(&self, nodes: Vec<TNode>) -> i32 {
        // let id = 0;
        // for (n in nodes) {
        // 	for (s in n.inputs) if (s.id >= id) id = s.id + 1;
        // 	for (s in n.outputs) if (s.id >= id) id = s.id + 1;
        // }
        // return id;
        unimplemented!()
    }

    fn input_linked(&self, canvas: TNodeCanvas, node_id: i32, i: i32) -> bool {
        // for (l in canvas.links) if (l.to_id == node_id && l.to_socket == i) return true;
        // return false;
        unimplemented!()
    }

    fn bake_elements(&self, ui: Ui) {
        // ui.g.end();
        // elementsBaked = true;
        // socketImage = Image.createRenderTarget(24, 24);
        // let g = socketImage.g2;
        // g.begin(true, 0x00000000);
        // g.color = 0xff000000;
        // graphics2.GraphicsExtension.fillCircle(g, 12, 12, 12);
        // g.color = 0xffffffff;
        // graphics2.GraphicsExtension.fillCircle(g, 12, 12, 9);
        // g.end();
        // ui.g.begin(false);
        unimplemented!()
    }

    pub fn node_canvas(&self, ui: Ui, canvas: TNodeCanvas) {
        // if (!elementsBaked) bakeElements(ui);

        // let wx = ui._windowX;
        // let wy = ui._windowY;
        // let _inputEnabled = ui.inputEnabled;
        // ui.inputEnabled = _inputEnabled && popupCommands == null;
        // let controls = onCanvasControl != null ? onCanvasControl() : {
        // 	panX: ui.inputDownR ? ui.inputDX : 0.0,
        // 	panY: ui.inputDownR ? ui.inputDY : 0.0,
        // 	zoom: -ui.inputWheelDelta / 10.0
        // };
        // socketReleased = false;

        // // Pan canvas
        // if (ui.inputEnabled && (controls.panX != 0.0 || controls.panY != 0.0)) {
        // 	panX += controls.panX / SCALE();
        // 	panY += controls.panY / SCALE();
        // }

        // // Zoom canvas
        // if (ui.inputEnabled && controls.zoom != 0.0) {
        // 	zoom += controls.zoom;
        // 	if (zoom < 0.1) zoom = 0.1;
        // 	else if (zoom > 1.0) zoom = 1.0;
        // 	zoom = Math.round(zoom * 10) / 10;
        // 	uiw = ui._w;
        // 	uih = ui._h;
        // }
        // scaleFactor = ui.SCALE();
        // ELEMENT_H = ui.t.ELEMENT_H + 2;
        // ui.setScale(SCALE()); // Apply zoomed scale
        // ui.elementsBaked = true;
        // ui.g.font = ui.ops.font;
        // ui.g.fontSize = ui.fontSize;

        // for (link in canvas.links) {
        // 	let from = getNode(canvas.nodes, link.from_id);
        // 	let to = getNode(canvas.nodes, link.to_id);
        // 	let fromX = from == null ? ui.inputX : wx + NODE_X(from) + NODE_W(from);
        // 	let fromY = from == null ? ui.inputY : wy + NODE_Y(from) + OUTPUT_Y(from.outputs, link.from_socket);
        // 	let toX = to == null ? ui.inputX : wx + NODE_X(to);
        // 	let toY = to == null ? ui.inputY : wy + NODE_Y(to) + INPUT_Y(canvas, to.inputs, link.to_socket) + OUTPUTS_H(to.outputs) + BUTTONS_H(to);

        // 	// Cull
        // 	let left = toX > fromX ? fromX : toX;
        // 	let right = toX > fromX ? toX : fromX;
        // 	let top = toY > fromY ? fromY : toY;
        // 	let bottom = toY > fromY ? toY : fromY;
        // 	if (right < 0 || left > wx + ui._windowW ||
        // 		bottom < 0 || top > wy + ui._windowH) {
        // 		continue;
        // 	}

        // 	// Snap to nearest socket
        // 	if (linkDrag == link) {
        // 		if (snapFromId != -1) {
        // 			fromX = snapX;
        // 			fromY = snapY;
        // 		}
        // 		if (snapToId != -1) {
        // 			toX = snapX;
        // 			toY = snapY;
        // 		}
        // 		snapFromId = snapToId = -1;

        // 		for (node in canvas.nodes) {
        // 			let inps = node.inputs;
        // 			let outs = node.outputs;
        // 			let nodeh = NODE_H(canvas, node);
        // 			let rx = wx + NODE_X(node) - LINE_H() / 2;
        // 			let ry = wy + NODE_Y(node) - LINE_H() / 2;
        // 			let rw = NODE_W(node) + LINE_H();
        // 			let rh = nodeh + LINE_H();
        // 			if (ui.getInputInRect(rx, ry, rw, rh)) {
        // 				if (from == null && node.id != to.id) { // Snap to output
        // 					for (i in 0...outs.length) {
        // 						let sx = wx + NODE_X(node) + NODE_W(node);
        // 						let sy = wy + NODE_Y(node) + OUTPUT_Y(outs, i);
        // 						let rx = sx - LINE_H() / 2;
        // 						let ry = sy - LINE_H() / 2;
        // 						if (ui.getInputInRect(rx, ry, LINE_H(), LINE_H())) {
        // 							snapX = sx;
        // 							snapY = sy;
        // 							snapFromId = node.id;
        // 							snapSocket = i;
        // 							break;
        // 						}
        // 					}
        // 				}
        // 				else if (to == null && node.id != from.id) { // Snap to input
        // 					for (i in 0...inps.length) {
        // 						let sx = wx + NODE_X(node);
        // 						let sy = wy + NODE_Y(node) + INPUT_Y(canvas, inps, i) + OUTPUTS_H(outs) + BUTTONS_H(node);
        // 						let rx = sx - LINE_H() / 2;
        // 						let ry = sy - LINE_H() / 2;
        // 						if (ui.getInputInRect(rx, ry, LINE_H(), LINE_H())) {
        // 							snapX = sx;
        // 							snapY = sy;
        // 							snapToId = node.id;
        // 							snapSocket = i;
        // 							break;
        // 						}
        // 					}
        // 				}
        // 			}
        // 		}
        // 	}

        // 	let selected = false;
        // 	for (n in nodesSelected) {
        // 		if (link.from_id == n.id || link.to_id == n.id) {
        // 			selected = true;
        // 			break;
        // 		}
        // 	}

        // 	drawLink(ui, fromX - wx, fromY - wy, toX - wx, toY - wy, selected);
        // }

        // for (node in canvas.nodes) {
        // 	// Cull
        // 	if (NODE_X(node) > ui._windowW || NODE_X(node) + NODE_W(node) < 0 ||
        // 		NODE_Y(node) > ui._windowH || NODE_Y(node) + NODE_H(canvas, node) < 0) {
        // 		if (!isSelected(node)) continue;
        // 	}

        // 	let inps = node.inputs;
        // 	let outs = node.outputs;

        // 	// Drag node
        // 	let nodeh = NODE_H(canvas, node);
        // 	if (ui.inputEnabled && ui.getInputInRect(wx + NODE_X(node) - LINE_H() / 2, wy + NODE_Y(node), NODE_W(node) + LINE_H(), LINE_H())) {
        // 		if (ui.inputStarted) {
        // 			if (ui.isShiftDown || ui.isCtrlDown) {
        // 				// Add to selection or deselect
        // 				isSelected(node) ?
        // 					nodesSelected.remove(node) :
        // 					nodesSelected.push(node);
        // 			}
        // 			else if (nodesSelected.length <= 1) {
        // 				// Selecting single node, otherwise wait for input release
        // 				nodesSelected = [node];
        // 			}
        // 			moveOnTop = node; // Place selected node on top
        // 			nodesDrag = true;
        // 			dragged = false;
        // 		}
        // 		else if (ui.inputReleased && !ui.isShiftDown && !ui.isCtrlDown && !dragged) {
        // 			// No drag performed, select single node
        // 			nodesSelected = [node];
        // 			if (onHeaderReleased != null) {
        // 				onHeaderReleased(node);
        // 			}
        // 		}
        // 	}
        // 	if (ui.inputStarted && ui.getInputInRect(wx + NODE_X(node) - LINE_H() / 2, wy + NODE_Y(node) - LINE_H() / 2, NODE_W(node) + LINE_H(), nodeh + LINE_H())) {
        // 		// Check sockets
        // 		if (linkDrag == null) {
        // 			for (i in 0...outs.length) {
        // 				let sx = wx + NODE_X(node) + NODE_W(node);
        // 				let sy = wy + NODE_Y(node) + OUTPUT_Y(outs, i);
        // 				if (ui.getInputInRect(sx - LINE_H() / 2, sy - LINE_H() / 2, LINE_H(), LINE_H())) {
        // 					// New link from output
        // 					let l: TNodeLink = { id: getLinkId(canvas.links), from_id: node.id, from_socket: i, to_id: -1, to_socket: -1 };
        // 					canvas.links.push(l);
        // 					linkDrag = l;
        // 					isNewLink = true;
        // 					break;
        // 				}
        // 			}
        // 		}
        // 		if (linkDrag == null) {
        // 			for (i in 0...inps.length) {
        // 				let sx = wx + NODE_X(node);
        // 				let sy = wy + NODE_Y(node) + INPUT_Y(canvas, inps, i) + OUTPUTS_H(outs) + BUTTONS_H(node);
        // 				if (ui.getInputInRect(sx - LINE_H() / 2, sy - LINE_H() / 2, LINE_H(), LINE_H())) {
        // 					// Already has a link - disconnect
        // 					for (l in canvas.links) {
        // 						if (l.to_id == node.id && l.to_socket == i) {
        // 							l.to_id = l.to_socket = -1;
        // 							linkDrag = l;
        // 							isNewLink = false;
        // 							break;
        // 						}
        // 					}
        // 					if (linkDrag != null) break;
        // 					// New link from input
        // 					let l: TNodeLink = {
        // 						id: getLinkId(canvas.links),
        // 						from_id: -1,
        // 						from_socket: -1,
        // 						to_id: node.id,
        // 						to_socket: i
        // 					};
        // 					canvas.links.push(l);
        // 					linkDrag = l;
        // 					isNewLink = true;
        // 					break;
        // 				}
        // 			}
        // 		}
        // 	}
        // 	else if (ui.inputReleased) {
        // 		if (snapToId != -1) { // Connect to input
        // 			// Force single link per input
        // 			for (l in canvas.links) {
        // 				if (l.to_id == snapToId && l.to_socket == snapSocket) {
        // 					canvas.links.remove(l);
        // 					break;
        // 				}
        // 			}
        // 			linkDrag.to_id = snapToId;
        // 			linkDrag.to_socket = snapSocket;
        // 			ui.changed = true;
        // 		}
        // 		else if (snapFromId != -1) { // Connect to output
        // 			linkDrag.from_id = snapFromId;
        // 			linkDrag.from_socket = snapSocket;
        // 			ui.changed = true;
        // 		}
        // 		else if (linkDrag != null) { // Remove dragged link
        // 			canvas.links.remove(linkDrag);
        // 			ui.changed = true;
        // 			if (onLinkDrag != null) {
        // 				onLinkDrag(linkDrag, isNewLink);
        // 			}
        // 		}
        // 		snapToId = snapFromId = -1;
        // 		linkDrag = null;
        // 		nodesDrag = false;
        // 	}
        // 	if (nodesDrag && isSelected(node) && !ui.inputDownR) {
        // 		if (ui.inputDX != 0 || ui.inputDY != 0) {
        // 			dragged = true;
        // 			node.x += Std.int(ui.inputDX / SCALE());
        // 			node.y += Std.int(ui.inputDY / SCALE());
        // 		}
        // 	}

        // 	drawNode(ui, node, canvas);
        // }

        // if (onCanvasReleased != null && ui.inputEnabled && (ui.inputReleased || ui.inputReleasedR) && !socketReleased) {
        // 	onCanvasReleased();
        // }

        // if (boxSelect) {
        // 	ui.g.color = 0x223333dd;
        // 	ui.g.fillRect(boxSelectX, boxSelectY, ui.inputX - boxSelectX - ui._windowX, ui.inputY - boxSelectY - ui._windowY);
        // 	ui.g.color = 0x773333dd;
        // 	ui.g.drawRect(boxSelectX, boxSelectY, ui.inputX - boxSelectX - ui._windowX, ui.inputY - boxSelectY - ui._windowY);
        // 	ui.g.color = 0xffffffff;
        // }
        // if (ui.inputEnabled && ui.inputStarted && !ui.isAltDown && linkDrag == null && !nodesDrag && !ui.changed) {
        // 	boxSelect = true;
        // 	boxSelectX = Std.int(ui.inputX - ui._windowX);
        // 	boxSelectY = Std.int(ui.inputY - ui._windowY);
        // }
        // else if (boxSelect && !ui.inputDown) {
        // 	boxSelect = false;
        // 	let nodes: Vec<TNode> = [];
        // 	let left = boxSelectX;
        // 	let top = boxSelectY;
        // 	let right = Std.int(ui.inputX - ui._windowX);
        // 	let bottom = Std.int(ui.inputY - ui._windowY);
        // 	if (left > right) {
        // 		let t = left;
        // 		left = right;
        // 		right = t;
        // 	}
        // 	if (top > bottom) {
        // 		let t = top;
        // 		top = bottom;
        // 		bottom = t;
        // 	}
        // 	for (n in canvas.nodes) {
        // 		if (NODE_X(n) + NODE_W(n) > left && NODE_X(n) < right &&
        // 			NODE_Y(n) + NODE_H(canvas, n) > top && NODE_Y(n) < bottom) {
        // 			nodes.push(n);
        // 		}
        // 	}
        // 	(ui.isShiftDown || ui.isCtrlDown) ? for (n in nodes) nodesSelected.push(n) : nodesSelected = nodes;
        // }

        // // Place selected node on top
        // if (moveOnTop != null) {
        // 	canvas.nodes.remove(moveOnTop);
        // 	canvas.nodes.push(moveOnTop);
        // 	moveOnTop = null;
        // }

        // // Node copy & paste
        // let cutSelected = false;
        // if (Zui.isCopy) {
        // 	let copyNodes: Vec<TNode> = [];
        // 	for (n in nodesSelected) {
        // 		if (excludeRemove.indexOf(n.type) >= 0) continue;
        // 		copyNodes.push(n);
        // 	}
        // 	let copyLinks: Vec<TNodeLink> = [];
        // 	for (l in canvas.links) {
        // 		let from = getNode(nodesSelected, l.from_id);
        // 		let to = getNode(nodesSelected, l.to_id);
        // 		if (from != null && excludeRemove.indexOf(from.type) == -1 &&
        // 			to != null && excludeRemove.indexOf(to.type) == -1) {
        // 			copyLinks.push(l);
        // 		}
        // 	}
        // 	let copyCanvas: TNodeCanvas = {
        // 		name: canvas.name,
        // 		nodes: copyNodes,
        // 		links: copyLinks
        // 	};
        // 	clipboard = haxe.Json.stringify(copyCanvas);
        // 	cutSelected = Zui.isCut;
        // }
        // if (Zui.isPaste && !ui.isTyping) {
        // 	let pasteCanvas: TNodeCanvas = null;
        // 	// Clipboard can contain non-json data
        // 	try {
        // 		pasteCanvas = haxe.Json.parse(clipboard);
        // 	}
        // 	catch(_) {}
        // 	if (pasteCanvas != null) {
        // 		for (l in pasteCanvas.links) {
        // 			// Assign unique link id
        // 			l.id = getLinkId(canvas.links);
        // 			canvas.links.push(l);
        // 		}
        // 		for (n in pasteCanvas.nodes) {
        // 			// Assign unique node id
        // 			let old_id = n.id;
        // 			n.id = getNodeId(canvas.nodes);
        // 			for (soc in n.inputs) {
        // 				soc.id = getSocketId(canvas.nodes);
        // 				soc.node_id = n.id;
        // 			}
        // 			for (soc in n.outputs) {
        // 				soc.id = getSocketId(canvas.nodes);
        // 				soc.node_id = n.id;
        // 			}
        // 			for (l in pasteCanvas.links) {
        // 				if (l.from_id == old_id) l.from_id = n.id;
        // 				else if (l.to_id == old_id) l.to_id = n.id;
        // 			}
        // 			n.x += 10;
        // 			n.y += 10;
        // 			canvas.nodes.push(n);
        // 		}
        // 		nodesSelected = pasteCanvas.nodes;
        // 		ui.changed = true;
        // 	}
        // }

        // // Select all nodes
        // if (ui.isCtrlDown && ui.key == input.KeyCode.A) {
        // 	nodesSelected = [];
        // 	for (n in canvas.nodes) nodesSelected.push(n);
        // }

        // // Node removal
        // if (ui.inputEnabled && (ui.isBackspaceDown || ui.isDeleteDown || cutSelected) && !ui.isTyping) {
        // 	let i = nodesSelected.length - 1;
        // 	while (i >= 0) {
        // 		let n = nodesSelected[i--];
        // 		if (excludeRemove.indexOf(n.type) >= 0) continue;
        // 		removeNode(n, canvas);
        // 		ui.changed = true;
        // 	}
        // }

        // ui.setScale(scaleFactor); // Restore non-zoomed scale
        // ui.elementsBaked = true;
        // ui.inputEnabled = _inputEnabled;

        // if (popupCommands != null) {
        // 	ui._x = popupX;
        // 	ui._y = popupY;
        // 	ui._w = popupW;

        // 	ui.fill(-6, -6, ui._w / ui.SCALE() + 12, popupH + 12, ui.t.ACCENT_SELECT_COL);
        // 	ui.fill(-5, -5, ui._w / ui.SCALE() + 10, popupH + 10, ui.t.SEPARATOR_COL);
        // 	popupCommands(ui);

        // 	let hide = (ui.inputStarted || ui.inputStartedR) && (ui.inputX - wx < popupX - 6 || ui.inputX - wx > popupX + popupW + 6 || ui.inputY - wy < popupY - 6 || ui.inputY - wy > popupY + popupH * ui.SCALE() + 6);
        // 	if (hide || ui.isEscapeDown) {
        // 		popupCommands = null;
        // 	}
        // }
        unimplemented!()
    }

    // Retrieve combo items for buttons of type ENUM
    // pub static let enumTexts: String->Vec<String> = null;

    #[inline]
    fn is_selected(&self, node: TNode) -> bool {
        // return nodesSelected.indexOf(node) >= 0;
        unimplemented!()
    }

    pub fn draw_node(&self, ui: Ui, node: TNode, canvas: TNodeCanvas) {
        // let wx = ui._windowX;
        // let wy = ui._windowY;
        // let uiX = ui._x;
        // let uiY = ui._y;
        // let uiW = ui._w;
        // let w = NODE_W(node);
        // let g = ui.g;
        // let h = NODE_H(canvas, node);
        // let nx = NODE_X(node);
        // let ny = NODE_Y(node);
        // let text = tr(node.name);
        // let lineh = LINE_H();

        // // Disallow input if node is overlapped by another node
        // _inputStarted = ui.inputStarted;
        // if (ui.inputStarted) {
        // 	for (i in (canvas.nodes.indexOf(node) + 1)...canvas.nodes.length) {
        // 		let n = canvas.nodes[i];
        // 		if (NODE_X(n) < ui.inputX - ui._windowX && NODE_X(n) + NODE_W(n) > ui.inputX - ui._windowX &&
        // 			NODE_Y(n) < ui.inputY - ui._windowY && NODE_Y(n) + NODE_H(canvas, n) > ui.inputY - ui._windowY) {
        // 			ui.inputStarted = false;
        // 			break;
        // 		}
        // 	}
        // }

        // // Outline
        // g.color = isSelected(node) ? ui.t.LABEL_COL : ui.t.CONTEXT_COL;
        // g.fillRect(nx - 1, ny - 1, w + 2, h + 2);

        // // Header
        // g.color = ui.t.WINDOW_BG_COL - 0x00060606;
        // g.fillRect(nx, ny, w, lineh);
        // g.color = node.color;
        // g.fillRect(nx, ny + lineh - p(3), w, p(3));

        // // Body
        // g.color = ui.t.WINDOW_BG_COL;
        // g.fillRect(nx, ny + lineh, w, h - lineh);

        // // Title
        // g.color = ui.t.LABEL_COL;
        // let textw = g.font.width(ui.fontSize, text);
        // g.drawString(text, nx + p(10), ny + p(6));
        // ny += lineh * 0.5;

        // // Outputs
        // for (out in node.outputs) {
        // 	ny += lineh;
        // 	g.color = out.color;
        // 	g.drawScaledImage(socketImage, nx + w - p(6), ny - p(3), p(12), p(12));
        // }
        // ny -= lineh * node.outputs.length;
        // g.color = ui.t.LABEL_COL;
        // for (out in node.outputs) {
        // 	ny += lineh;
        // 	let strw = ui.ops.font.width(ui.fontSize, tr(out.name));
        // 	g.drawString(tr(out.name), nx + w - strw - p(12), ny - p(3));
        // 	if (onSocketReleased != null && ui.inputEnabled && (ui.inputReleased || ui.inputReleasedR)) {
        // 		if (ui.inputX > wx + nx && ui.inputX < wx + nx + w && ui.inputY > wy + ny && ui.inputY < wy + ny + lineh) {
        // 			onSocketReleased(out);
        // 			socketReleased = true;
        // 		}
        // 	}
        // }

        // // Buttons
        // let nhandle = handle.nest(node.id);
        // ny -= lineh / 3; // Fix align
        // for (buti in 0...node.buttons.length) {
        // 	let but = node.buttons[buti];

        // 	if (but.type == "RGBA") {
        // 		ny += lineh; // 18 + 2 separator
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		let val = node.outputs[but.output].default_value;
        // 		nhandle.color = Color.fromFloats(val[0], val[1], val[2]);
        // 		Ext.colorWheel(ui, nhandle);
        // 		val[0] = nhandle.color.R;
        // 		val[1] = nhandle.color.G;
        // 		val[2] = nhandle.color.B;
        // 	}
        // 	else if (but.type == "VECTOR") {
        // 		ny += lineh;
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		let min = but.min != null ? but.min : 0.0;
        // 		let max = but.max != null ? but.max : 1.0;
        // 		let textOff = ui.t.TEXT_OFFSET;
        // 		ui.t.TEXT_OFFSET = 6;
        // 		ui.text(tr(but.name));
        // 		but.default_value[0] = ui.slider(nhandle.nest(buti).nest(0, {value: but.default_value[0]}), "X", min, max, true, 100, true, Left);
        // 		but.default_value[1] = ui.slider(nhandle.nest(buti).nest(1, {value: but.default_value[1]}), "Y", min, max, true, 100, true, Left);
        // 		but.default_value[2] = ui.slider(nhandle.nest(buti).nest(2, {value: but.default_value[2]}), "Z", min, max, true, 100, true, Left);
        // 		ui.t.TEXT_OFFSET = textOff;
        // 		if (but.output != null) node.outputs[but.output].default_value = but.default_value;
        // 		ny += lineh * 3;
        // 	}
        // 	else if (but.type == "VALUE") {
        // 		ny += lineh;
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		let soc = node.outputs[but.output];
        // 		let min = but.min != null ? but.min : 0.0;
        // 		let max = but.max != null ? but.max : 1.0;
        // 		let prec = but.precision != null ? but.precision : 100.0;
        // 		let textOff = ui.t.TEXT_OFFSET;
        // 		ui.t.TEXT_OFFSET = 6;
        // 		soc.default_value = ui.slider(nhandle.nest(buti, {value: soc.default_value}), "Value", min, max, true, prec, true, Left);
        // 		ui.t.TEXT_OFFSET = textOff;
        // 	}
        // 	else if (but.type == "STRING") {
        // 		ny += lineh;
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		let soc = but.output != null ? node.outputs[but.output] : null;
        // 		but.default_value = ui.textInput(nhandle.nest(buti, {text: soc != null ? soc.default_value : but.default_value != null ? but.default_value : ""}), tr(but.name));
        // 		if (soc != null) soc.default_value = but.default_value;
        // 	}
        // 	else if (but.type == "ENUM") {
        // 		ny += lineh;
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		let texts = Std.is(but.data, Array) ? [for (s in cast(but.data, Vec<Dynamic>)) tr(s)] : enumTexts(node.type);
        // 		let buthandle = nhandle.nest(buti);
        // 		buthandle.position = but.default_value;
        // 		but.default_value = ui.combo(buthandle, texts, tr(but.name));
        // 	}
        // 	else if (but.type == "BOOL") {
        // 		ny += lineh;
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		but.default_value = ui.check(nhandle.nest(buti, {selected: but.default_value}), tr(but.name));
        // 	}
        // 	else if (but.type == "CUSTOM") { // Calls external fn for custom button drawing
        // 		ny += lineh;
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		let dot = but.name.lastIndexOf("."); // TNodeButton.name specifies external fn path
        // 		let fn = Reflect.field(Type.resolveClass(but.name.substr(0, dot)), but.name.substr(dot + 1));
        // 		fn(ui, this, node);
        // 		ny += lineh * (but.height - 1); // TNodeButton.height specifies vertical button size
        // 	}
        // }
        // ny += lineh / 3; // Fix align

        // // Inputs
        // for (i in 0...node.inputs.length) {
        // 	let inp = node.inputs[i];
        // 	ny += lineh;
        // 	g.color = inp.color;
        // 	g.drawScaledImage(socketImage, nx - p(6), ny - p(3), p(12), p(12));
        // 	let isLinked = inputLinked(canvas, node.id, i);
        // 	if (!isLinked && inp.type == "VALUE") {
        // 		ui._x = nx + p(6);
        // 		ui._y = ny - p(9);
        // 		ui._w = Std.int(w - p(6));
        // 		let soc = inp;
        // 		let min = soc.min != null ? soc.min : 0.0;
        // 		let max = soc.max != null ? soc.max : 1.0;
        // 		let prec = soc.precision != null ? soc.precision : 100.0;
        // 		let textOff = ui.t.TEXT_OFFSET;
        // 		ui.t.TEXT_OFFSET = 6;
        // 		soc.default_value = ui.slider(nhandle.nest(maxButtons).nest(i, {value: soc.default_value}), tr(inp.name), min, max, true, prec, true, Left);
        // 		ui.t.TEXT_OFFSET = textOff;
        // 	}
        // 	else if (!isLinked && inp.type == "STRING") {
        // 		ui._x = nx + p(6);
        // 		ui._y = ny - p(9);
        // 		ui._w = Std.int(w - p(6));
        // 		let soc = inp;
        // 		let textOff = ui.t.TEXT_OFFSET;
        // 		ui.t.TEXT_OFFSET = 6;
        // 		soc.default_value = ui.textInput(nhandle.nest(maxButtons).nest(i, {text: soc.default_value}), tr(inp.name), Left);
        // 		ui.t.TEXT_OFFSET = textOff;
        // 	}
        // 	else if (!isLinked && inp.type == "RGBA") {
        // 		g.color = ui.t.LABEL_COL;
        // 		g.drawString(tr(inp.name), nx + p(12), ny - p(3));
        // 		let soc = inp;
        // 		g.color = 0xff000000;
        // 		g.fillRect(nx + w - p(38), ny - p(6), p(36), p(18));
        // 		g.color = Color.fromFloats(soc.default_value[0], soc.default_value[1], soc.default_value[2]);
        // 		let rx = nx + w - p(37);
        // 		let ry = ny - p(5);
        // 		let rw = p(34);
        // 		let rh = p(16);
        // 		g.fillRect(rx, ry, rw, rh);
        // 		let ix = ui.inputX - wx;
        // 		let iy = ui.inputY - wy;
        // 		if (ui.inputStarted && ix > rx && iy > ry && ix < rx + rw && iy < ry + rh) {
        // 			_inputStarted = ui.inputStarted = false;
        // 			rgbaPopup(ui, nhandle, soc.default_value, Std.int(rx), Std.int(ry + ui.ELEMENT_H()));
        // 		}
        // 	}
        // 	else if (!isLinked && inp.type == "VECTOR" && inp.display == 1) {
        // 		g.color = ui.t.LABEL_COL;
        // 		g.drawString(tr(inp.name), nx + p(12), ny - p(3));
        // 		ny += lineh / 2;
        // 		ui._x = nx;
        // 		ui._y = ny;
        // 		ui._w = w;
        // 		let min = inp.min != null ? inp.min : 0.0;
        // 		let max = inp.max != null ? inp.max : 1.0;
        // 		let textOff = ui.t.TEXT_OFFSET;
        // 		ui.t.TEXT_OFFSET = 6;
        // 		inp.default_value[0] = ui.slider(nhandle.nest(maxButtons).nest(i).nest(0, {value: inp.default_value[0]}), "X", min, max, true, 100, true, Left);
        // 		inp.default_value[1] = ui.slider(nhandle.nest(maxButtons).nest(i).nest(1, {value: inp.default_value[1]}), "Y", min, max, true, 100, true, Left);
        // 		inp.default_value[2] = ui.slider(nhandle.nest(maxButtons).nest(i).nest(2, {value: inp.default_value[2]}), "Z", min, max, true, 100, true, Left);
        // 		ui.t.TEXT_OFFSET = textOff;
        // 		ny += lineh * 2.5;
        // 	}
        // 	else {
        // 		g.color = ui.t.LABEL_COL;
        // 		g.drawString(tr(inp.name), nx + p(12), ny - p(3));
        // 	}
        // 	if (onSocketReleased != null && ui.inputEnabled && (ui.inputReleased || ui.inputReleasedR)) {
        // 		if (ui.inputX > wx + nx && ui.inputX < wx + nx + w && ui.inputY > wy + ny && ui.inputY < wy + ny + lineh) {
        // 			onSocketReleased(inp);
        // 			socketReleased = true;
        // 		}
        // 	}
        // }

        // ui._x = uiX;
        // ui._y = uiY;
        // ui._w = uiW;
        // ui.inputStarted = _inputStarted;
        unimplemented!()
    }

    pub fn rgba_popup(&self, ui: Ui, nhandle: Handle, val: Vec<f32>, x: i32, y: i32) {
        // popup(x, y, Std.int(140 * scaleFactor), Std.int(ui.t.ELEMENT_H * 9), fn(ui: Ui) {
        // 	nhandle.color = Color.fromFloats(val[0], val[1], val[2]);
        // 	Ext.colorWheel(ui, nhandle, false, null, false);
        // 	val[0] = nhandle.color.R; val[1] = nhandle.color.G; val[2] = nhandle.color.B;
        // });
        unimplemented!()
    }

    pub fn draw_link(
        &self,
        ui: Ui,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        highlight: bool, /* = false*/
    ) {
        // let g = ui.g;
        // let c1: Color = ui.t.LABEL_COL;
        // let c2: Color = ui.t.ACCENT_SELECT_COL;
        // g.color = highlight ? Color.fromBytes(c1.Rb, c1.Gb, c1.Bb, 210) : Color.fromBytes(c2.Rb, c2.Gb, c2.Bb, 210);
        // if (ui.t.LINK_STYLE == Line) {
        // 	g.drawLine(x1, y1, x2, y2, 1.0);
        // 	g.color = highlight ? Color.fromBytes(c1.Rb, c1.Gb, c1.Bb, 150) : Color.fromBytes(c2.Rb, c2.Gb, c2.Bb, 150); // AA
        // 	g.drawLine(x1 + 0.5, y1, x2 + 0.5, y2, 1.0);
        // 	g.drawLine(x1 - 0.5, y1, x2 - 0.5, y2, 1.0);
        // 	g.drawLine(x1, y1 + 0.5, x2, y2 + 0.5, 1.0);
        // 	g.drawLine(x1, y1 - 0.5, x2, y2 - 0.5, 1.0);
        // }
        // else if (ui.t.LINK_STYLE == CubicBezier) {
        // 	g.drawCubicBezier([x1, x1 + Math.abs(x1 - x2) / 2, x2 - Math.abs(x1 - x2) / 2, x2], [y1, y1, y2, y2], 30, highlight ? 2.0 : 1.0);
        // }
        unimplemented!()
    }

    pub fn remove_node(&self, n: TNode, canvas: TNodeCanvas) {
        // if (n == null) return;
        // let i = 0;
        // while (i < canvas.links.length) {
        // 	let l = canvas.links[i];
        // 	if (l.from_id == n.id || l.to_id == n.id) {
        // 		canvas.links.splice(i, 1);
        // 	}
        // 	else i++;
        // }
        // canvas.nodes.remove(n);
        // if (onNodeRemove != null) {
        // 	onNodeRemove(n);
        // }
        unimplemented!()
    }

    // let popupX = 0;
    // let popupY = 0;
    // let popupW = 0;
    // let popupH = 0;
    // let popupCommands: Fn(Ui) = null;

    fn popup(&self, x: i32, y: i32, w: i32, h: i32, commands: Box<dyn Fn(Ui)>) {
        // popupX = x;
        // popupY = y;
        // popupW = w;
        // popupH = h;
        // popupCommands = commands;
        unimplemented!()
    }
}

pub struct CanvasControl {
    pan_x: f32,
    pan_y: f32,
    zoom: f32,
}

pub struct TNodeCanvas {
    name: String,
    nodes: Vec<TNode>,
    links: Vec<TNodeLink>,
}

pub struct TNode {
    id: i32,
    name: String,
    kind: String,
    x: f32,
    y: f32,
    inputs: Vec<TNodeSocket>,
    outputs: Vec<TNodeSocket>,
    buttons: Vec<TNodeButton>,
    color: i32,
    width: Option<f32>,
}

pub struct TNodeSocket {
    id: i32,
    node_id: i32,
    name: String,
    kind: String,
    color: i32,
    // default_value: Dynamic;
    min: Option<f32>,
    max: Option<f32>,
    precision: Option<f32>,
    display: Option<i32>,
}

pub struct TNodeLink {
    id: i32,
    from_id: i32,
    from_socket: i32,
    to_id: i32,
    to_socket: i32,
}

pub struct TNodeButton {
    name: String,
    kind: String,
    output: Option<i32>,
    // default_value: Dynamic,
    // data: Dynamic,
    min: Option<f32>,
    max: Option<f32>,
    precision: Option<f32>,
    height: Option<f32>,
}
