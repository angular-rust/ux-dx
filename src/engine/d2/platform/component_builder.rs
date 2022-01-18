use std::collections::HashMap;

pub struct Field;

pub struct ComponentBuilder {
    // static
    name_cache: HashMap<String, String>, // = HashMap<String,String>::new();
    // static
    next_id: u32,
}

impl ComponentBuilder {
    // // static
    // pub fn build(&self) -> Vec<Field> {
    //     let pos = Context::currentPos();
    //     let cl = Context::getLocalClass().get();

    //     let name = Context::makeExpr(self.getComponentName(cl), pos);
    //     // let componentType = TPath({pack: cl.pack, name: cl.name, params: []});

    //     // let fields = Macros.buildFields(macro {
    //     //     let public__static__inline__NAME = $name;
    //     // });

    //     // // Only override get_name if this component directly extends a componentBase and creates a
    //     // // new namespace
    //     // if (extendsComponentBase(cl)) {
    //     //     fields = fields.concat(Macros.buildFields(macro {
    //     //         fn override__private__get_name () ->String {
    //     //             return $name;
    //     //         }
    //     //     }));
    //     // }

    //     // return fields.concat(Context.getBuildFields());
    //     unimplemented!()
    // }

    // static
    // fn getComponentName(&self, cl: ClassType) -> String {
    //     // Traverse up to the last non-component base
    //     loop {
    //         if Self::extendsComponentBase(cl) {
    //             break;
    //         }
    //         cl = cl.superClass.t.get();
    //     }

    //     // Look up the ID, otherwise generate one
    //     let fullName = cl.pack.concat([cl.name]).join(".");
    //     let name = self._nameCache.get(fullName);
    //     if name.is_none() {
    //         name = cl.name + "_" + self._nextId;
    //         self._nameCache.set(fullName, name);
    //         self._nextId += 1;
    //     }

    //     name
    // }

    // // static
    // fn extendsComponentBase(cl: ClassType) {
    //     let superClass = cl.superClass.t.get();

    //     superClass.meta.has(":componentBase")
    // }
}
