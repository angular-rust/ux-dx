/// Utilities for macros.
pub struct Macros {}

impl Macros {
    // /// Creates a list of fields from a block expression.
    // // static
    // pub fn buildFields (block :Expr) ->Vec<Field>
    // {
    //     let fields :Vec<Field> = Vec::new();
    //     match (block.expr) {
    //         case EBlock(exprs):
    //             let metas = Vec::new();
    //             for (expr in exprs) {
    //                 match (expr.expr) {
    //                     case EMeta(meta, e):
    //                         metas.push(meta);
    //                     case EVars(vars):
    //                         for (v in vars) {
    //                             fields.push({
    //                                 name: getFieldName(v.name),
    //                                 doc: None,
    //                                 access: getAccess(v.name),
    //                                 kind: FVar(v.type, v.expr),
    //                                 pos: v.expr.pos,
    //                                 meta: metas,
    //                             });
    //                         }
    //                         metas = Vec::new();
    //                     case EFunction(name, f):
    //                         fields.push({
    //                             name: getFieldName(name),
    //                             doc: None,
    //                             access: getAccess(name),
    //                             kind: FFun(f),
    //                             pos: f.expr.pos,
    //                             meta: metas,
    //                         });
    //                         metas = Vec::new();
    //                     default:
    //                 }
    //             }
    //         default:
    //     }
    //     return fields;
    // }

    // // static
    // fn getAccess (name :String) ->Vec<Access>
    // {
    //     let result = Vec::new();
    //     for (token in name.split("__")) {
    //         let access = match (token) {
    //             case "public": APublic;
    //             case "private": APrivate;
    //             case "static": AStatic;
    //             case "override": AOverride;
    //             case "dynamic": ADynamic;
    //             case "inline": AInline;
    //             default: None;
    //         }
    //         if (access.is_some()) {
    //             result.push(access);
    //         }
    //     }
    //     return result;
    // }

    // // static
    // fn getFieldName (name :String) ->String
    // {
    //     let parts = name.split("__");
    //     return parts[parts.len()-1];
    // }
}
