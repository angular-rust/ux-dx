use super::*;

// import haxe.macro.Context;
// import haxe.macro.Expr;
// import haxe.macro.ExprTools;

static i: i32 = 0;

pub struct Id {}

impl Id {
    pub fn pos() -> i32 {
        // return macro $v{i++};
        unimplemented!()
    }

    pub fn handle(ops: HandleOptions /* = null*/) -> Handle {
        // let code = "zui.Zui.Handle.global.nest(zui.Id.pos()," + ExprTools.toString(ops) + ")";
        // return Context.parse(code, Context.currentPos());
        unimplemented!()
    }
}
