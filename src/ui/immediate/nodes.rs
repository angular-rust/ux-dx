#![allow(unused_variables)]

use winit::event::VirtualKeyCode;

use crate::{foundation::colorspace::Color, prelude::Lighten, uiid};

use super::*;

pub struct Nodes {
    pub nodesDrag: bool,
    pub nodesSelected: Vec<TNode>,
    pub panX: f32,
    pub panY: f32,
    pub zoom: f32,
    pub uiw: f32,
    pub uih: f32,
    pub input_started: bool,
    scaleFactor: f32,
    element_h: u32,
    dragged: bool,
    moveOnTop: Option<TNode>,
    linkDrag: Option<TNodeLink>,
    isNewLink: bool,
    snapFromId: Option<u32>,
    snapToId: Option<u32>,
    snapSocket: u32,
    snapX: f32,
    snapY: f32,
    handle: Handle,
    // static
    elementsBaked: bool,
    // static
    socketImage: Option<Image>,
    // static
    socketReleased: bool,
    // static
    clipboard: String,
    // static
    boxSelect: bool,
    // static
    boxSelectX: i32,
    // static
    boxSelectY: i32,
    // static inline
    maxButtons: i32,

    popupX: i32,
    popupY: i32,
    popupW: i32,
    popupH: i32,

    nodeId: Option<u32>,

    // No removal for listed node types
    // static
    pub excludeRemove: Vec<String>,
    // static
    pub onLinkDrag: Option<Box<dyn Fn(&TNodeLink, bool)>>,
    // static
    pub onHeaderReleased: Option<Box<dyn Fn(&TNode)>>,
    // static
    pub onSocketReleased: Option<Box<dyn Fn(&TNodeSocket)>>,
    // static
    pub onCanvasReleased: Option<Box<dyn Fn()>>,
    // static
    pub onNodeRemove: Option<Box<dyn Fn(&TNode)>>,
    // static
    pub onCanvasControl: Option<Box<dyn Fn() -> CanvasControl>>, // Pan, zoom
    popupCommands: Option<Box<dyn Fn(&Ui)>>,
}

impl Default for Nodes {
    fn default() -> Self {
        Nodes {
            nodesDrag: false,
            nodesSelected: Vec::new(),
            panX: 0.0,
            panY: 0.0,
            zoom: 1.0,
            uiw: 0.0,
            uih: 0.0,
            input_started: false,
            scaleFactor: 1.0,
            element_h: 25,
            dragged: false,
            moveOnTop: None,
            linkDrag: None,
            isNewLink: false,
            snapFromId: None,
            snapToId: None,
            snapSocket: 0,
            snapX: 0.0,
            snapY: 0.0,
            handle: Id::handle(uiid!(), None),
            elementsBaked: false,
            socketImage: None,
            socketReleased: false,
            clipboard: "".into(),
            boxSelect: false,
            boxSelectX: 0,
            boxSelectY: 0,
            maxButtons: 9,

            popupX: 0,
            popupY: 0,
            popupW: 0,
            popupH: 0,

            nodeId: None,

            // No removal for listed node types
            excludeRemove: Vec::new(),
            onLinkDrag: None,
            onHeaderReleased: None,
            onSocketReleased: None,
            onCanvasReleased: None,
            onNodeRemove: None,
            onCanvasControl: None, // Pan, zoom

            popupCommands: None,
        }
    }
}

impl Nodes {
    // static dynamic
    // id: String, vars: HashMap<String, String> = None
    #[cfg(feature = "translate")]
    pub fn tr(id: String) -> String {
        id
    }

    // static inline
    #[cfg(not(feature = "translate"))]
    pub fn tr(id: String) -> String {
        id
    }

    pub fn new() {}

    #[inline]
    pub fn scale(&self) -> f32 {
        self.scaleFactor * self.zoom
    }

    #[inline]
    pub fn pan_x(&self) -> f32 {
        let zoomPan = (1.0 - self.zoom) * self.uiw as f32 / 2.5;
        self.panX * self.scale() + zoomPan
    }

    #[inline]
    pub fn pan_y(&self) -> f32 {
        let zoomPan = (1.0 - self.zoom) * self.uih as f32 / 2.5;
        self.panY * self.scale() + zoomPan
    }

    #[inline]
    pub fn line_h(&self) -> f32 {
        self.element_h as f32 * self.scale()
    }

    fn buttons_h(&self, node: &TNode) -> f32 {
        let mut h = 0.0;
        for but in node.buttons.iter() {
            match but.type_ {
                TNodeType::Rgba => h += 235.0 * self.scale(),
                TNodeType::Vector => h += self.line_h() as f32 * 4.0,
                TNodeType::Custom => {
                    // possible should be unwrapped to 0.0 // DV
                    h += self.line_h() as f32 * but.height.unwrap_or(1.0);
                }
                _ => h += self.line_h() as f32,
            }
        }

        h
    }

    fn outputs_h(&self, sockets: &Vec<TNodeSocket>, length: Option<u32> /* = None*/) -> f32 {
        let mut h = 0.0;
        let len = if let Some(length) = length {
            length as usize
        } else {
            sockets.len()
        };

        for idx in 0..len {
            h += self.line_h() as f32;
        }

        h
    }

    fn inputs_h(
        &self,
        canvas: &TNodeCanvas,
        sockets: &Vec<TNodeSocket>,
        length: Option<u32>, /* = None*/
    ) -> f32 {
        let mut h = 0.0;
        let len = if let Some(length) = length {
            length as usize
        } else {
            sockets.len()
        };

        for idx in 0..len {
            if sockets[idx].type_ == TNodeType::Vector
                && sockets[idx].display == 1
                && !self.input_linked(canvas, sockets[idx].node_id, idx as u32)
            {
                h += self.line_h() as f32 * 4.0;
            } else {
                h += self.line_h() as f32;
            }
        }

        h
    }

    #[inline]
    fn node_h(&self, canvas: &TNodeCanvas, node: &TNode) -> f32 {
        self.line_h() as f32 * 1.2
            + self.inputs_h(canvas, &node.inputs, None) as f32
            + self.outputs_h(&node.outputs, None) as f32
            + self.buttons_h(node) as f32
    }

    #[inline]
    fn node_w(&self, node: &TNode) -> f32 {
        let w = if let Some(width) = node.width {
            width * self.scale()
        } else {
            140.0 * self.scale()
        };

        w
    }

    #[inline]
    fn node_x(&self, node: &TNode) -> f32 {
        node.x * self.scale() + self.pan_x()
    }

    #[inline]
    fn node_y(&self, node: &TNode) -> f32 {
        node.y * self.scale() + self.pan_y()
    }

    #[inline]
    fn input_y(&self, canvas: &TNodeCanvas, sockets: &Vec<TNodeSocket>, pos: u32) -> f32 {
        self.line_h() as f32 * 1.62 + self.inputs_h(canvas, sockets, Some(pos))
    }

    #[inline]
    fn output_y(&self, sockets: &Vec<TNodeSocket>, pos: u32) -> f32 {
        self.line_h() as f32 * 1.62 + self.outputs_h(sockets, Some(pos))
    }

    #[inline]
    pub fn p(&self, f: f32) -> f32 {
        f * self.scale()
    }

    pub fn node(&self, nodes: &Vec<TNode>, id: u32) -> Option<TNode> {
        for node in nodes.iter() {
            if node.id == id {
                return Some(node.clone());
            }
        }

        None
    }

    pub fn node_id(&mut self, nodes: &Vec<TNode>) -> u32 {
        if self.nodeId.is_none() {
            let mut node_id = 0;
            for n in nodes {
                if node_id < n.id {
                    node_id = n.id;
                }
            }
            self.nodeId = Some(node_id);
        }

        // fetch next node id
        match &mut self.nodeId {
            Some(node_id) => {
                let prev = *node_id;
                *node_id += 1;

                prev
            }
            None => 0,
        }
    }

    pub fn link_id(&self, links: &Vec<TNodeLink>) -> u32 {
        let mut id = 0;
        for l in links {
            if l.id >= id {
                id = l.id + 1;
            }
        }

        id
    }

    pub fn socket_id(&self, nodes: &Vec<TNode>) -> u32 {
        let mut id = 0;
        for n in nodes {
            for s in n.inputs.iter() {
                if s.id >= id {
                    id = s.id + 1;
                }
            }

            for s in n.outputs.iter() {
                if s.id >= id {
                    id = s.id + 1;
                }
            }
        }

        id
    }

    fn input_linked(&self, canvas: &TNodeCanvas, node_id: u32, socket: u32) -> bool {
        for link in canvas.links.iter() {
            if link.to_id.map(|to_id| to_id == node_id).unwrap_or_default()
                && link
                    .to_socket
                    .map(|to_socket| to_socket == socket)
                    .unwrap_or_default()
            {
                return true;
            }
        }
        false
    }

    fn bake_elements(&mut self, ui: &Ui) {
        // ui.g.end();
        self.elementsBaked = true;
        // socketImage = Image.create_render_target(24, 24);
        // let g = socketImage.g2;
        // g.begin(true, 0x00000000);
        // g.color = 0xff000000;
        // graphics2.GraphicsExtension.fillCircle(g, 12, 12, 12);
        // g.color = 0xffffffff;
        // graphics2.GraphicsExtension.fillCircle(g, 12, 12, 9);
        // g.end();
        // ui.g.begin(false);
    }

    pub fn node_canvas(&mut self, ui: &mut Ui, canvas: &mut TNodeCanvas) {
        if !self.elementsBaked {
            self.bake_elements(ui);
        }

        let wx = ui.window_x;
        let wy = ui.window_y;
        let _input_enabled = ui.input_enabled;
        ui.input_enabled = _input_enabled && self.popupCommands.is_none();

        let controls = {
            if let Some(ref onCanvasControl) = self.onCanvasControl {
                onCanvasControl()
            } else {
                CanvasControl {
                    pan_x: if ui.input_down_r { ui.input_dx } else { 0.0 },
                    pan_y: if ui.input_down_r { ui.input_dy } else { 0.0 },
                    zoom: -ui.input_wheel_delta as f32 / 10.0,
                }
            }
        };
        self.socketReleased = false;

        // Pan canvas
        if ui.input_enabled && (controls.pan_x != 0.0 || controls.pan_y != 0.0) {
            self.panX += controls.pan_x / self.scale();
            self.panY += controls.pan_y / self.scale();
        }

        // Zoom canvas
        if ui.input_enabled && controls.zoom != 0.0 {
            self.zoom += controls.zoom;
            if self.zoom < 0.1 {
                self.zoom = 0.1;
            } else if self.zoom > 1.0 {
                self.zoom = 1.0;
            }
            self.zoom = (self.zoom * 10.0).round() / 10.0;
            self.uiw = ui.w;
            self.uih = ui.h;
        }
        self.scaleFactor = ui.scale();
        self.element_h = ui.theme.element_h + 2;
        ui.set_scale(self.scale()); // Apply zoomed scale

        // ui.elements_baked = true; // DV

        ui.painter.set_font(ui.ops.font);
        ui.painter.set_font_size(ui.font_size);

        for link in canvas.links.iter() {
            let from = link.from_id.and_then(|id| self.node(&canvas.nodes, id));
            let to = link.to_id.and_then(|id| self.node(&canvas.nodes, id));

            let mut fromX = if let Some(from) = &from {
                wx + self.node_x(from) + self.node_w(from)
            } else {
                ui.input_x
            };

            let mut fromY = if let Some(from) = &from {
                wy + self.node_y(from)
                    + self.output_y(&from.outputs, link.from_socket.unwrap_or_default())
            } else {
                ui.input_y
            };

            let mut toX = if let Some(to) = &to {
                wx + self.node_x(to)
            } else {
                ui.input_x
            };

            let mut toY = if let Some(to) = &to {
                wy + self.node_y(to)
                    + self.input_y(canvas, &to.inputs, link.to_socket.unwrap_or_default())
                    + self.outputs_h(&to.outputs, None)
                    + self.buttons_h(to)
            } else {
                ui.input_y
            };

            // Cull
            let left = if toX > fromX { fromX } else { toX };
            let right = if toX > fromX { toX } else { fromX };
            let top = if toY > fromY { fromY } else { toY };
            let bottom = if toY > fromY { toY } else { fromY };
            if right < 0.0 || left > wx + ui.window_w || bottom < 0.0 || top > wy + ui.window_h {
                continue;
            }

            if let Some(linkDrag) = self.linkDrag {
                // Snap to nearest socket
                if linkDrag == *link {
                    if self.snapFromId.is_some() {
                        fromX = self.snapX;
                        fromY = self.snapY;
                    }

                    if self.snapToId.is_some() {
                        toX = self.snapX;
                        toY = self.snapY;
                    }

                    self.snapFromId = None;
                    self.snapToId = None;

                    for node in canvas.nodes.iter() {
                        let inps = &node.inputs;
                        let outs = &node.outputs;
                        let nodeh = self.node_h(canvas, node);
                        let rx = wx + self.node_x(node) - self.line_h() / 2.0;
                        let ry = wy + self.node_y(node) - self.line_h() / 2.0;
                        let rw = self.node_w(node) + self.line_h();
                        let rh = nodeh + self.line_h();
                        if ui.input_in_rect(rx, ry, rw, rh, 1.0) {
                            if let Some(to) = &to {
                                if from.is_none() && node.id != to.id {
                                    // Snap to output
                                    for i in 0..outs.len() {
                                        let sx = wx + self.node_x(node) + self.node_w(node);
                                        let sy =
                                            wy + self.node_y(node) + self.output_y(outs, i as u32);
                                        let rx = sx - self.line_h() / 2.0;
                                        let ry = sy - self.line_h() / 2.0;
                                        if ui.input_in_rect(
                                            rx,
                                            ry,
                                            self.line_h(),
                                            self.line_h(),
                                            1.0,
                                        ) {
                                            self.snapX = sx;
                                            self.snapY = sy;
                                            self.snapFromId = Some(node.id);
                                            self.snapSocket = i as u32;
                                            break;
                                        }
                                    }
                                }
                            } else if let Some(from) = &from {
                                if to.is_none() && node.id != from.id {
                                    // Snap to input
                                    for i in 0..inps.len() {
                                        let sx = wx + self.node_x(node);
                                        let sy = wy
                                            + self.node_y(node)
                                            + self.input_y(canvas, inps, i as u32)
                                            + self.outputs_h(outs, None)
                                            + self.buttons_h(node);
                                        let rx = sx - self.line_h() / 2.0;
                                        let ry = sy - self.line_h() / 2.0;
                                        if ui.input_in_rect(
                                            rx,
                                            ry,
                                            self.line_h(),
                                            self.line_h(),
                                            1.0,
                                        ) {
                                            self.snapX = sx;
                                            self.snapY = sy;
                                            self.snapToId = Some(node.id);
                                            self.snapSocket = i as u32;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let mut selected = false;
            for n in self.nodesSelected.iter() {
                if link.from_id == Some(n.id) || link.to_id == Some(n.id) {
                    selected = true;
                    break;
                }
            }

            self.draw_link(ui, fromX - wx, fromY - wy, toX - wx, toY - wy, selected);
        }

        for node in canvas.nodes.iter() {
            // Cull
            if self.node_x(node) > ui.window_w
                || self.node_x(node) + self.node_w(node) < 0.0
                || self.node_y(node) > ui.window_h
                || self.node_y(node) + self.node_h(canvas, node) < 0.0
            {
                if !self.is_selected(node) {
                    continue;
                }
            }

            let inps = &node.inputs;
            let outs = &node.outputs;

            // Drag node
            let nodeh = self.node_h(canvas, node);
            if ui.input_enabled
                && ui.input_in_rect(
                    wx + self.node_x(node) - self.line_h() / 2.0,
                    wy + self.node_y(node),
                    self.node_w(node) + self.line_h(),
                    self.line_h(),
                    1.0,
                )
            {
                if ui.input_started {
                    if ui.is_shift_down || ui.is_ctrl_down {
                        // Add to selection or deselect
                        if self.is_selected(node) {
                            // self.nodesSelected.remove(node); //DV
                        } else {
                            self.nodesSelected.push(node.clone());
                        }
                    } else if self.nodesSelected.len() <= 1 {
                        // Selecting single node, otherwise wait for input release
                        self.nodesSelected = vec![node.clone()];
                    }
                    self.moveOnTop = Some(node.clone()); // Place selected node on top
                    self.nodesDrag = true;
                    self.dragged = false;
                } else if ui.input_released
                    && !ui.is_shift_down
                    && !ui.is_ctrl_down
                    && !self.dragged
                {
                    // No drag performed, select single node
                    self.nodesSelected = vec![node.clone()];
                    if let Some(onHeaderReleased) = &self.onHeaderReleased {
                        onHeaderReleased(node);
                    }
                }
            }

            if ui.input_started
                && ui.input_in_rect(
                    wx + self.node_x(node) - self.line_h() / 2.0,
                    wy + self.node_y(node) - self.line_h() / 2.0,
                    self.node_w(node) + self.line_h(),
                    nodeh + self.line_h(),
                    1.0,
                )
            {
                // Check sockets
                if self.linkDrag.is_none() {
                    for i in 0..outs.len() {
                        let sx = wx + self.node_x(node) + self.node_w(node);
                        let sy = wy + self.node_y(node) + self.output_y(outs, i as u32);
                        if ui.input_in_rect(
                            sx - self.line_h() / 2.0,
                            sy - self.line_h() / 2.0,
                            self.line_h(),
                            self.line_h(),
                            1.0,
                        ) {
                            // New link from output
                            let l = TNodeLink {
                                id: self.link_id(&canvas.links),
                                from_id: Some(node.id),
                                from_socket: Some(i as u32),
                                to_id: None,
                                to_socket: None,
                            };
                            canvas.links.push(l);
                            self.linkDrag = Some(l);
                            self.isNewLink = true;
                            break;
                        }
                    }
                }

                if self.linkDrag.is_none() {
                    for socket in 0..inps.len() {
                        let sx = wx + self.node_x(node);
                        let sy = wy
                            + self.node_y(node)
                            + self.input_y(canvas, inps, socket as u32)
                            + self.outputs_h(outs, None)
                            + self.buttons_h(node);
                        if ui.input_in_rect(
                            sx - self.line_h() / 2.0,
                            sy - self.line_h() / 2.0,
                            self.line_h(),
                            self.line_h(),
                            1.0,
                        ) {
                            // Already has a link - disconnect
                            for link in canvas.links.iter_mut() {
                                if link.to_id == Some(node.id)
                                    && link.to_socket == Some(socket as u32)
                                {
                                    link.to_id = None;
                                    link.to_socket = None;
                                    self.linkDrag = Some(link.clone());
                                    self.isNewLink = false;
                                    break;
                                }
                            }

                            if self.linkDrag.is_some() {
                                break;
                            }

                            // New link from input
                            let l = TNodeLink {
                                id: self.link_id(&canvas.links),
                                from_id: None,
                                from_socket: None,
                                to_id: Some(node.id),
                                to_socket: Some(socket as u32),
                            };
                            canvas.links.push(l);
                            self.linkDrag = Some(l);
                            self.isNewLink = true;
                            break;
                        }
                    }
                }
            } else if ui.input_released {
                if self.snapToId.is_some() {
                    // Connect to input
                    // Force single link per input
                    for l in canvas.links.iter() {
                        if l.to_id == self.snapToId && l.to_socket == Some(self.snapSocket) {
                            // canvas.links.remove(l); // DV
                            break;
                        }
                    }
                    // self.linkDrag.to_id = self.snapToId; // DV
                    // self.linkDrag.to_socket = self.snapSocket; // DV
                    ui.changed = true;
                } else if self.snapFromId.is_some() {
                    // Connect to output
                    // self.linkDrag.from_id = self.snapFromId; // DV
                    // self.linkDrag.from_socket = self.snapSocket; // DV
                    ui.changed = true;
                } else if self.linkDrag.is_some() {
                    // Remove dragged link
                    // canvas.links.remove(self.linkDrag); // DV
                    ui.changed = true;
                    if let Some(onLinkDrag) = &self.onLinkDrag {
                        onLinkDrag(&self.linkDrag.unwrap(), self.isNewLink);
                    }
                }
                self.snapToId = None;
                self.snapFromId = None;
                self.linkDrag = None;
                self.nodesDrag = false;
            }

            if self.nodesDrag && self.is_selected(node) && !ui.input_down_r {
                if ui.input_dx != 0.0 || ui.input_dy != 0.0 {
                    self.dragged = true;
                    // node.x += ui.input_dx / self.scale();
                    // node.y += ui.input_dy / self.scale();
                }
            }

            // self.draw_node(ui, node, canvas); // DV
        }

        if ui.input_enabled && (ui.input_released || ui.input_released_r) && !self.socketReleased {
            if let Some(onCanvasReleased) = &self.onCanvasReleased {
                onCanvasReleased();
            }
        }

        if self.boxSelect {
            ui.painter.set_color(Color::rgba(0x33, 0x33, 0xdd, 0x22));
            ui.painter.fill_rect(
                self.boxSelectX as f32,
                self.boxSelectY as f32,
                ui.input_x - self.boxSelectX as f32 - ui.window_x,
                ui.input_y - self.boxSelectY as f32 - ui.window_y,
            );
            ui.painter.set_color(Color::rgba(0x33, 0x33, 0xdd, 0x77));
            ui.painter.draw_rect(
                self.boxSelectX as f32,
                self.boxSelectY as f32,
                ui.input_x - self.boxSelectX as f32 - ui.window_x,
                ui.input_y - self.boxSelectY as f32 - ui.window_y,
                1.0,
            );
            ui.painter.set_color(color::WHITE);
        }

        if ui.input_enabled
            && ui.input_started
            && !ui.is_alt_down
            && self.linkDrag.is_none()
            && !self.nodesDrag
            && !ui.changed
        {
            self.boxSelect = true;
            self.boxSelectX = (ui.input_x - ui.window_x) as i32;
            self.boxSelectY = (ui.input_y - ui.window_y) as i32;
        } else if self.boxSelect && !ui.input_down {
            self.boxSelect = false;
            let mut nodes: Vec<TNode> = Vec::new();
            let mut left = self.boxSelectX;
            let mut top = self.boxSelectY;
            let mut right = (ui.input_x - ui.window_x) as i32;
            let mut bottom = (ui.input_y - ui.window_y) as i32;

            if left > right {
                let t = left;
                left = right;
                right = t;
            }

            if top > bottom {
                let t = top;
                top = bottom;
                bottom = t;
            }

            for n in canvas.nodes.iter() {
                if self.node_x(n) + self.node_w(n) > left as f32
                    && self.node_x(n) < right as f32
                    && self.node_y(n) + self.node_h(&canvas, n) > top as f32
                    && self.node_y(n) < bottom as f32
                {
                    nodes.push(n.clone());
                }
            }

            if ui.is_shift_down || ui.is_ctrl_down {
                for n in nodes.iter() {
                    self.nodesSelected.push(n.clone())
                }
            } else {
                self.nodesSelected = nodes;
            }
        }

        // Place selected node on top
        if self.moveOnTop.is_some() {
            // canvas.nodes.remove(self.moveOnTop); // DV
            // canvas.nodes.push(self.moveOnTop); // DV
            self.moveOnTop = None;
        }

        // Node copy & paste
        let mut cutSelected = false;
        if ui.is_copy {
            let mut copyNodes: Vec<TNode> = vec![];
            for n in self.nodesSelected.iter() {
                // if self.excludeRemove.indexOf(n.type_) >= 0 {
                //     continue;
                // } // DV
                copyNodes.push(n.clone());
            }

            let copyLinks: Vec<TNodeLink> = vec![];
            for link in canvas.links.iter() {
                let from = link
                    .from_id
                    .and_then(|id| self.node(&self.nodesSelected, id));
                let to = link.to_id.and_then(|id| self.node(&self.nodesSelected, id));

                // if from.is_some() && self.excludeRemove.indexOf(from.type_) == -1 &&
                // 	to.is_some() && self.excludeRemove.indexOf(to.type_) == -1 {
                // 	copyLinks.push(l);
                // } // DV
            }

            let copyCanvas = TNodeCanvas {
                name: canvas.name.clone(),
                nodes: copyNodes,
                links: copyLinks,
            };

            // self.clipboard = Json.stringify(copyCanvas); // DV
            cutSelected = ui.is_cut;
        }

        if ui.is_paste && !ui.is_typing {
            let mut pasteCanvas: Option<TNodeCanvas> = None;
            // Clipboard can contain non-json data
            // try {
            // 	pasteCanvas = Json.parse(clipboard);
            // }
            // catch(_) {} // DV

            if let Some(ref mut pasteCanvas) = pasteCanvas {
                for l in pasteCanvas.links.iter_mut() {
                    // Assign unique link id
                    l.id = self.link_id(&canvas.links);
                    canvas.links.push(*l);
                }

                for n in pasteCanvas.nodes.iter_mut() {
                    // Assign unique node id
                    let old_id = n.id;
                    n.id = self.node_id(&canvas.nodes);
                    for soc in n.inputs.iter_mut() {
                        soc.id = self.socket_id(&canvas.nodes);
                        soc.node_id = n.id;
                    }

                    for soc in n.outputs.iter_mut() {
                        soc.id = self.socket_id(&canvas.nodes);
                        soc.node_id = n.id;
                    }

                    for link in pasteCanvas.links.iter_mut() {
                        if link.from_id == Some(old_id) {
                            link.from_id = Some(n.id);
                        } else if link.to_id == Some(old_id) {
                            link.to_id = Some(n.id);
                        }
                    }
                    n.x += 10.0;
                    n.y += 10.0;
                    canvas.nodes.push(n.clone());
                }
                self.nodesSelected = pasteCanvas.nodes.clone();
                ui.changed = true;
            }
        }

        // Select all nodes
        if ui.is_ctrl_down && ui.key == Some(VirtualKeyCode::A) {
            self.nodesSelected = vec![];
            for n in canvas.nodes.iter() {
                self.nodesSelected.push(n.clone());
            }
        }

        // Node removal
        if ui.input_enabled
            && (ui.is_backspace_down || ui.is_delete_down || cutSelected)
            && !ui.is_typing
        {
            let mut i = self.nodesSelected.len() as i32 - 1;
            while i >= 0 {
                let n = &self.nodesSelected[i as usize];
                i -= 1;

                // if self.excludeRemove.indexOf(n.type_) >= 0 {
                //     continue;
                // } // DV

                self.remove_node(n, canvas);
                ui.changed = true;
            }
        }

        // Restore non-zoomed scale
        ui.set_scale(self.scaleFactor);
        // ui.elementsBaked = true;
        ui.input_enabled = _input_enabled;

        if let Some(popupCommands) = &self.popupCommands {
            ui.x = self.popupX as f32;
            ui.y = self.popupY as f32;
            ui.w = self.popupW as f32;

            ui.fill(
                -6.0,
                -6.0,
                ui.w / ui.scale() + 12.0,
                self.popupH as f32 + 12.0,
                ui.theme.accent_select_col,
            );
            ui.fill(
                -5.0,
                -5.0,
                ui.w / ui.scale() + 10.0,
                self.popupH as f32 + 10.0,
                ui.theme.separator_col,
            );

            popupCommands(ui);

            let hide = (ui.input_started || ui.input_started_r)
                && (ui.input_x - wx < self.popupX as f32 - 6.0
                    || ui.input_x - wx > self.popupX as f32 + self.popupW as f32 + 6.0
                    || ui.input_y - wy < self.popupY as f32 - 6.0
                    || ui.input_y - wy
                        > self.popupY as f32 + self.popupH as f32 * ui.scale() + 6.0);
            if hide || ui.is_escape_down {
                self.popupCommands = None;
            }
        }
    }

    // Retrieve combo items for buttons of type ENUM
    // static
    // pub enumTexts: String->Vec<String> = None;

    #[inline]
    fn is_selected(&self, node: &TNode) -> bool {
        // self.nodesSelected.indexOf(node) >= 0
        unimplemented!()
    }

    pub fn draw_node(&mut self, ui: &mut Ui, node: &mut TNode, canvas: &TNodeCanvas) {
        let wx = ui.window_x;
        let wy = ui.window_y;
        let uiX = ui.x;
        let uiY = ui.y;
        let uiW = ui.w;
        let w = self.node_w(node);

        let h = self.node_h(canvas, node);
        let nx = self.node_x(node);
        let mut ny = self.node_y(node);
        let text = Self::tr(node.name.clone());
        let lineh = self.line_h();

        // Disallow input if node is overlapped by another node
        self.input_started = ui.input_started;
        if ui.input_started {
            // for i in (canvas.nodes.indexOf(node) + 1)..canvas.nodes.len() {
            // 	let n = canvas.nodes[i];
            // 	if self.node_x(n) < ui.input_x - ui.window_x && self.node_x(n) + self.node_w(n) > ui.input_x - ui.window_x &&
            //         self.node_y(n) < ui.input_y - ui.window_y && self.node_y(n) + self.node_h(canvas, n) > ui.input_y - ui.window_y {
            // 		ui.input_started = false;
            // 		break;
            // 	}
            // } // DV
        }

        // Outline
        if self.is_selected(node) {
            ui.painter.set_color(ui.theme.label_col);
        } else {
            ui.painter.set_color(ui.theme.context_col);
        }

        ui.painter.fill_rect(nx - 1.0, ny - 1.0, w + 2.0, h + 2.0);

        // Header
        ui.painter.set_color(ui.theme.window_bg_col.darken(6.0));
        ui.painter.fill_rect(nx, ny, w, lineh);
        ui.painter.set_color(node.color);
        ui.painter
            .fill_rect(nx, ny + lineh - self.p(3.0), w, self.p(3.0));

        // Body
        ui.painter.set_color(ui.theme.window_bg_col);
        ui.painter.fill_rect(nx, ny + lineh, w, h - lineh);

        // Title
        ui.painter.set_color(ui.theme.label_col);

        if let Some((width, _)) = ui.painter.measure(text.as_str()) {
            ui.painter
                .draw_string(text.as_str(), nx + self.p(10.0), ny + self.p(6.0));
        }

        ny += lineh * 0.5;

        if let Some(ref socket_image) = self.socketImage {
            // Outputs
            for out in node.outputs.iter() {
                ny += lineh;
                ui.painter.set_color(out.color);
                ui.painter.draw_scaled_image(
                    socket_image,
                    nx + w - self.p(6.0),
                    ny - self.p(3.0),
                    self.p(12.0),
                    self.p(12.0),
                );
            }
        }

        ny -= lineh * node.outputs.len() as f32;
        ui.painter.set_color(ui.theme.label_col);
        for out in node.outputs.iter() {
            ny += lineh;

            if let Some((strw, _)) = ui.painter.measure(Self::tr(out.name.clone()).as_str()) {
                ui.painter.draw_string(
                    Self::tr(out.name.clone()).as_str(),
                    nx + w - strw - self.p(12.0),
                    ny - self.p(3.0),
                );
            }

            if let Some(onSocketReleased) = &self.onSocketReleased {
                if ui.input_enabled && (ui.input_released || ui.input_released_r) {
                    if ui.input_x > wx + nx
                        && ui.input_x < wx + nx + w
                        && ui.input_y > wy + ny
                        && ui.input_y < wy + ny + lineh
                    {
                        onSocketReleased(out);
                        self.socketReleased = true;
                    }
                }
            }
        }

        // Buttons
        let nhandle = self.handle.nest(node.id as u64, None);
        ny -= lineh / 3.0; // Fix align
        for buti in 0..node.buttons.len() {
            let but = &mut node.buttons[buti];

            if but.type_ == TNodeType::Rgba {
                ny += lineh; // 18 + 2 separator
                ui.x = nx;
                ui.y = ny;
                ui.w = w;
                if let Some(output) = but.output {
                    if let TNodeValue::Rgba(red, green, blue, alpha) =
                        node.outputs[output as usize].default_value
                    {
                        // props handling
                        match nhandle.props.write() {
                            Ok(mut props) => {
                                props.color = Color {
                                    red,
                                    green,
                                    blue,
                                    alpha,
                                };
                            }
                            Err(e) => panic!("RwLock poisoned"),
                        }
                    }

                    ui.color_wheel(&nhandle, false, None, true);
                    // val[0] = props.color.red;
                    // val[1] = props.color.green;
                    // val[2] = props.color.blue;
                }
            } else if but.type_ == TNodeType::Vector {
                ny += lineh;
                ui.x = nx;
                ui.y = ny;
                ui.w = w;
                let min = but.min.unwrap_or_default();
                let max = but.max.unwrap_or(1.0);
                let textOff = ui.theme.text_offset;
                ui.theme.text_offset = 6;
                ui.text(Self::tr(but.name.clone()).as_str(), Align::Left, None);

                // but.default_value[0] = ui.slider(&nhandle.nest(buti as u64, None).nest(0, Some(HandleOptions{value: but.default_value[0], ..Default::default()})), "X", min, max, true, 100.0, true, Align::Left, true);
                // but.default_value[1] = ui.slider(&nhandle.nest(buti as u64, None).nest(1, Some(HandleOptions{value: but.default_value[1], ..Default::default()})), "Y", min, max, true, 100.0, true, Align::Left, true);
                // but.default_value[2] = ui.slider(&nhandle.nest(buti as u64, None).nest(2, Some(HandleOptions{value: but.default_value[2], ..Default::default()})), "Z", min, max, true, 100.0, true, Align::Left, true);

                ui.theme.text_offset = textOff;
                if let Some(output) = but.output {
                    node.outputs[output as usize].default_value = but.default_value.clone();
                }
                ny += lineh * 3.0;
            } else if but.type_ == TNodeType::Value {
                ny += lineh;
                ui.x = nx;
                ui.y = ny;
                ui.w = w;
            // let soc = node.outputs[but.output];
            // let min = but.min.unwrap_or_default();
            // let max = but.max.unwrap_or(1.0);
            // let prec = but.precision.unwrap_or(100.0);
            // let textOff = ui.theme.text_offset;
            // ui.theme.text_offset = 6;
            // soc.default_value = ui.slider(nhandle.nest(buti as u64, Some(HandleOptions{value: soc.default_value, ..Default::default()})), "Value", min, max, true, prec, true, Align::Left, true);
            // ui.theme.text_offset = textOff;
            } else if but.type_ == TNodeType::String {
                ny += lineh;
                ui.x = nx;
                ui.y = ny;
                ui.w = w;
            // let soc = if let Some(output) = but.output {Some(node.outputs[output as usize])} else {None};
            // but.default_value = ui.text_input(nhandle.nest(buti, Some(HandleOptions{text: soc != None ? soc.default_value : but.default_value != None ? but.default_value : "", ..Default::default()})), tr(but.name));
            // if let Some(soc) = soc {
            //     soc.default_value = but.default_value;
            // }
            } else if but.type_ == TNodeType::Enum {
                ny += lineh;
                ui.x = nx;
                ui.y = ny;
                ui.w = w;
            // let texts = Std.is(but.data, Array) ? [for (s in cast(but.data, Vec<TNodeValue>)) Self::tr(s)] : enumTexts(node.type);
            // let buthandle = nhandle.nest(buti);
            // buthandle.position = but.default_value;
            // but.default_value = ui.combo(buthandle, texts, Self::tr(but.name));
            } else if but.type_ == TNodeType::Bool {
                ny += lineh;
                ui.x = nx;
                ui.y = ny;
                ui.w = w;
                let selected = match but.default_value {
                    TNodeValue::Bool(val) => Some(val),
                    _ => Some(false),
                };
                but.default_value = TNodeValue::Bool(ui.check(
                    &nhandle.nest(
                        buti as u64,
                        Some(HandleOptions {
                            selected,
                            ..Default::default()
                        }),
                    ),
                    Self::tr(but.name.clone()).as_str(),
                ));
            } else if but.type_ == TNodeType::Custom {
                // Calls external fn for custom button drawing
                ny += lineh;
                ui.x = nx;
                ui.y = ny;
                ui.w = w;
                // let dot = but.name.lastIndexOf("."); // TNodeButton.name specifies external fn path
                // let fn = Reflect.field(Type.resolveClass(but.name.substr(0, dot)), but.name.substr(dot + 1));
                // fn(ui, this, node); // DV
                ny += lineh * (but.height.unwrap_or_default() - 1.0); // TNodeButton.height specifies vertical button size
            }
        }
        ny += lineh / 3.0; // Fix align

        // Inputs
        for i in 0..node.inputs.len() {
            let inp = &mut node.inputs[i];
            ny += lineh;
            ui.painter.set_color(inp.color);
            // ui.painter.draw_scaled_image(self.socketImage, nx - self.p(6.0), ny - self.p(3.0), self.p(12.0), self.p(12.0));
            // let isLinked = self.input_linked(canvas, node.id, i as u32);
            // if !isLinked && inp.type_ == TNodeType::Value {
            // 	ui.x = nx + self.p(6.0);
            // 	ui.y = ny - self.p(9.0);
            // 	ui.w = w - self.p(6.0);
            // 	let soc = inp;
            // 	let min = soc.min.unwrap_or_default();
            // 	let max = soc.max.unwrap_or(1.0);
            // 	let prec = soc.precision.unwrap_or(100.0);
            // 	let textOff = ui.theme.text_offset;
            // 	ui.theme.text_offset = 6;
            // 	soc.default_value = ui.slider(&nhandle.nest(self.maxButtons).nest(i, Some(HandleOptions{value: soc.default_value, ..Default::default()})), tr(inp.name), min, max, true, prec, true, Align::Left, true);
            // 	ui.theme.text_offset = textOff;
            // } else if !isLinked && inp.type_ == TNodeType::String {
            // 	ui.x = nx + self.p(6.0);
            // 	ui.y = ny - self.p(9.0);
            // 	ui.w = w - self.p(6.0);
            // 	let soc = inp;
            // 	let textOff = ui.theme.text_offset;
            // 	ui.theme.text_offset = 6;
            // 	soc.default_value = ui.text_input(&nhandle.nest(self.maxButtons).nest(i, Some(HandleOptions{text: soc.default_value, ..Default::default()})), tr(inp.name), Left);
            // 	ui.theme.text_offset = textOff;
            // } else if !isLinked && inp.type_ == TNodeType::Rgba {
            // 	ui.painter.color = ui.theme.label_col;
            // 	ui.painter.draw_string(Self::tr(inp.name), nx + self.p(12.0), ny - self.p(3.0));
            // 	let soc = inp;
            // 	ui.painter.set_color(color::BLACK);
            // 	ui.painter.fill_rect(nx + w - self.p(38.0), ny - self.p(6.0), self.p(36.0), self.p(18.0));
            // 	ui.painter.set_color(Color { red: soc.default_value[0], green: soc.default_value[1], blue: soc.default_value[2], alpha: 1.});
            // 	let rx = nx + w - self.p(37.0);
            // 	let ry = ny - self.p(5.0);
            // 	let rw = self.p(34.0);
            // 	let rh = self.p(16.0);
            // 	ui.painter.fill_rect(rx, ry, rw, rh);
            // 	let ix = ui.input_x - wx;
            // 	let iy = ui.input_y - wy;
            // 	if ui.input_started && ix > rx && iy > ry && ix < rx + rw && iy < ry + rh {
            // 		self.input_started = false;
            //         ui.input_started = false;
            // 		self.rgba_popup(ui, nhandle, soc.default_value, rx as i32, (ry + ui.element_h()) as i32);
            // 	}
            // } else if !isLinked && inp.type_ == TNodeType::Vector && inp.display == 1 {
            // 	ui.painter.set_color(ui.theme.label_col);
            // 	ui.painter.draw_string(Self::tr(inp.name).as_str(), nx + self.p(12.0), ny - self.p(3.0));
            // 	ny += lineh / 2.0;
            // 	ui.x = nx;
            // 	ui.y = ny;
            // 	ui.w = w;
            // 	let min = inp.min.unwrap_or_default();
            // 	let max = inp.max.unwrap_or(1.0);
            // 	let textOff = ui.theme.text_offset;
            // 	ui.theme.text_offset = 6;
            // 	inp.default_value[0] = ui.slider(&nhandle.nest(self.maxButtons as u64, None).nest(i as u64, None).nest(0, Some(HandleOptions{value: inp.default_value[0], ..Default::default()})), "X", min, max, true, 100, true, Align::Left, true);
            // 	inp.default_value[1] = ui.slider(&nhandle.nest(self.maxButtons as u64, None).nest(i as u64, None).nest(1, Some(HandleOptions{value: inp.default_value[1], ..Default::default()})), "Y", min, max, true, 100, true, Align::Left, true);
            // 	inp.default_value[2] = ui.slider(&nhandle.nest(self.maxButtons as u64, None).nest(i as u64, None).nest(2, Some(HandleOptions{value: inp.default_value[2], ..Default::default()})), "Z", min, max, true, 100, true, Align::Left, true);
            // 	ui.theme.text_offset = textOff;
            // 	ny += lineh * 2.5;
            // } else {
            // 	ui.painter.set_color(ui.theme.label_col);
            // 	ui.painter.draw_string(Self::tr(inp.name).as_str(), nx + self.p(12.0), ny - self.p(3.0));
            // }

            if let Some(onSocketReleased) = &self.onSocketReleased {
                if ui.input_enabled && (ui.input_released || ui.input_released_r) {
                    if ui.input_x > wx + nx
                        && ui.input_x < wx + nx + w
                        && ui.input_y > wy + ny
                        && ui.input_y < wy + ny + lineh
                    {
                        onSocketReleased(&inp);
                        self.socketReleased = true;
                    }
                }
            }
        }

        ui.x = uiX;
        ui.y = uiY;
        ui.w = uiW;
        ui.input_started = self.input_started;
    }

    pub fn rgba_popup(&mut self, ui: &mut Ui, nhandle: Handle, val: Vec<f32>, x: i32, y: i32) {
        match nhandle.props.write() {
            Ok(mut props) => {
                props.color = Color {
                    red: val[0],
                    green: val[1],
                    blue: val[2],
                    alpha: 1.,
                }
            }
            Err(e) => panic!("RwLock poisoned"),
        }

        // self.popup(
        //     x,
        //     y,
        //     (140.0 * self.scaleFactor) as i32,
        //     (ui.theme.element_h * 9) as i32,
        //     |ui| {
        //         ui.color_wheel(&nhandle, false, None, false);
        //         // val[0] = nhandle.color.red;
        //         // val[1] = nhandle.color.green;
        //         // val[2] = nhandle.color.blue;
        //     },
        // );
    }

    pub fn draw_link(
        &self,
        ui: &Ui,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        highlight: bool, /* = false*/
    ) {
        let c1: Color = ui.theme.label_col;
        let c2: Color = ui.theme.accent_select_col;

        if highlight {
            ui.painter.set_color(Color {
                red: c1.red,
                green: c1.green,
                blue: c1.blue,
                alpha: 210.0 / 255.,
            });
        } else {
            ui.painter.set_color(Color {
                red: c2.red,
                green: c2.green,
                blue: c2.blue,
                alpha: 210.0 / 255.,
            });
        }

        if ui.theme.link_style == LinkStyle::Line {
            ui.painter.draw_line(x1, y1, x2, y2, 1.0);

            if highlight {
                ui.painter.set_color(Color {
                    red: c1.red,
                    green: c1.green,
                    blue: c1.blue,
                    alpha: 150.0 / 255.0,
                });
            } else {
                ui.painter.set_color(Color {
                    red: c2.red,
                    green: c2.green,
                    blue: c2.blue,
                    alpha: 150.0 / 255.0,
                });
            }

            ui.painter.draw_line(x1 + 0.5, y1, x2 + 0.5, y2, 1.0);
            ui.painter.draw_line(x1 - 0.5, y1, x2 - 0.5, y2, 1.0);
            ui.painter.draw_line(x1, y1 + 0.5, x2, y2 + 0.5, 1.0);
            ui.painter.draw_line(x1, y1 - 0.5, x2, y2 - 0.5, 1.0);
        } else if ui.theme.link_style == LinkStyle::CubicBezier {
            let strength = if highlight { 2.0 } else { 1.0 };
            // ui.painter.drawCubicBezier([x1, x1 + (x1 - x2).abs() / 2.0, x2 - (x1 - x2).abs() / 2.0, x2], [y1, y1, y2, y2], 30, strength);
        }
    }

    pub fn remove_node(&self, n: &TNode, canvas: &TNodeCanvas) {
        // if n == None {
        //     return;
        // }

        let mut i = 0;
        while i < canvas.links.len() {
            let link = canvas.links[i];
            if link.from_id == Some(n.id) || link.to_id == Some(n.id) {
                // canvas.links.splice(i, 1); // DV
            } else {
                i += 1;
            }
        }

        // canvas.nodes.remove(n);
        if let Some(onNodeRemove) = &self.onNodeRemove {
            onNodeRemove(&n);
        }
    }

    fn popup<F>(&mut self, x: i32, y: i32, w: i32, h: i32, commands: F)
    where
        F: Fn(&Ui) + 'static,
    {
        self.popupX = x;
        self.popupY = y;
        self.popupW = w;
        self.popupH = h;
        self.popupCommands = Some(Box::new(commands));
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TNodeType {
    Value,
    String,
    Rgba,
    Vector,
    Enum,
    Bool,
    Custom,
    None,
}

impl Default for TNodeType {
    fn default() -> Self {
        TNodeType::None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TNodeValue {
    None,
    Value(f32),
    String(String),
    Rgba(f32, f32, f32, f32),
    Vector(f32, f32, f32),
    Enum,
    Bool(bool),
    Custom,
}

impl Default for TNodeValue {
    fn default() -> Self {
        TNodeValue::None
    }
}

#[derive(Clone, Default)]
pub struct TNode {
    id: u32,
    name: String,
    type_: TNodeType,
    x: f32,
    y: f32,
    inputs: Vec<TNodeSocket>,
    outputs: Vec<TNodeSocket>,
    buttons: Vec<TNodeButton>,
    color: Color,
    width: Option<f32>,
}

#[derive(Clone, Default)]
pub struct TNodeSocket {
    id: u32,
    node_id: u32,
    name: String,
    type_: TNodeType,
    color: Color,
    default_value: TNodeValue,
    min: Option<f32>,
    max: Option<f32>,
    precision: Option<f32>,
    display: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TNodeLink {
    id: u32,
    from_id: Option<u32>,
    from_socket: Option<u32>,
    to_id: Option<u32>,
    to_socket: Option<u32>,
}

#[derive(Clone, Default)]
pub struct TNodeButton {
    name: String,
    type_: TNodeType,
    output: Option<u32>,
    default_value: TNodeValue,
    data: TNodeValue,
    min: Option<f32>,
    max: Option<f32>,
    precision: Option<f32>,
    height: Option<f32>,
}
