use std::{cmp, collections::HashMap, fmt, marker::PhantomData};

use crate::engine::d2::util::{BitSets, Disposable};

use super::{
    display::{EmitterSprite, FillSprite, ImageSprite, PatternSprite, Sprite, TextSprite},
    scene::Scene2D,
    swf::{Flipbook, MoviePlayer, MovieSprite},
    Component, SpeedAdjuster,
};

#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct EntityID(u64);

impl EntityID {
    pub fn entity(&self) -> Entity {
        unimplemented!()
    }
}

pub trait EntityManager<T> {
    fn get(&self) -> Option<T>;
}

impl EntityManager<SpeedAdjuster> for Entity {
    fn get(&self) -> Option<SpeedAdjuster> {
        Some(SpeedAdjuster::default())
    }
}

impl EntityManager<Scene2D> for Entity {
    fn get(&self) -> Option<Scene2D> {
        Some(Scene2D::default())
    }
}

impl EntityManager<MoviePlayer> for Entity {
    fn get(&self) -> Option<MoviePlayer> {
        Some(MoviePlayer::default())
    }
}

impl EntityManager<MovieSprite> for Entity {
    fn get(&self) -> Option<MovieSprite> {
        Some(MovieSprite::default())
    }
}

impl EntityManager<Flipbook> for Entity {
    fn get(&self) -> Option<Flipbook> {
        Some(Flipbook::default())
    }
}

impl EntityManager<PatternSprite> for Entity {
    fn get(&self) -> Option<PatternSprite> {
        Some(PatternSprite::default())
    }
}

impl EntityManager<EmitterSprite> for Entity {
    fn get(&self) -> Option<EmitterSprite> {
        Some(EmitterSprite::default())
    }
}

impl EntityManager<FillSprite> for Entity {
    fn get(&self) -> Option<FillSprite> {
        Some(FillSprite::default())
    }
}

impl EntityManager<ImageSprite> for Entity {
    fn get(&self) -> Option<ImageSprite> {
        Some(ImageSprite::default())
    }
}

impl EntityManager<TextSprite> for Entity {
    fn get(&self) -> Option<TextSprite> {
        Some(TextSprite::default())
    }
}

impl EntityManager<Sprite> for Entity {
    fn get(&self) -> Option<Sprite> {
        Some(Sprite::default())
    }
}

impl EntityManager<Component> for Entity {
    fn get(&self) -> Option<Component> {
        Some(Component::default())
    }
}

trait EntityHolder {
    fn entity(&self) -> Entity {
        unimplemented!()
    }
}

struct EntityFactory<T>(PhantomData<T>)
where
    T: EntityHolder;

impl<T> EntityFactory<T>
where
    T: EntityHolder,
{
    pub fn create() -> T {
        unimplemented!()
    }
}

/// A node in the entity hierarchy, and a collection of components.
///
/// To iterate over the hierarchy, use the parent, first_child, next and first_component fields. For
/// example:
///
// ```
// // Iterate over entity's children
// let child = entity.first_child;
// while (child.is_some()) {
//     let next = child.next; // Store in case the child is removed in process()
//     process(child);
//     child = next;
// }
// ```
#[derive(Default, Copy, Clone, Debug)]
pub struct Entity {
    pub id: EntityID,
    /// This entity's parent
    pub parent: Option<EntityID>,

    /// This entity's first child
    pub first_child: Option<EntityID>,

    /// This entity's next sibling, for iteration
    pub next: Option<EntityID>,

    /// This entity's first component
    pub first_component: Option<Component>,
    // /// Maps String -> Component. Usually you would use a Map here, but I'm dropping down to plain
    // /// Object/Dictionary for the quickest possible lookups in this critical part.
    // components: HashMap<String, Component>,

    //Use set_zorder() to set this value instead of z_order = x
    pub z_order: i32,          // = 0;
    pub order_of_arrival: i32, // = 1;

    // static
    pub global_order_of_arrival: i32, // = 1;
}

impl Entity {
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a component to this entity. Any previous component of this type will be replaced.
    /// @returns This instance, for chaining.
    pub fn add(&self, component: Component) -> &Entity {
        // Remove the component from any previous owner. Don't just call dispose, which has
        // additional behavior in some components (like Disposer).

        todo!("should get entity from storage");
        // if let Some(ref owner) = component.owner {
        //     owner.remove(&component);
        // }

        // let name = component.name;
        // let prev = self.getComponent(name);
        // if let Some(prev) = prev {
        //     // Remove the previous component under this name
        //     self.remove(prev);
        // }

        // self.components.insert(name, component);

        // // Append it to the component list
        // let tail = None;
        // let p = self.first_component;
        // while let Some(ref val) = p {
        //     tail = p;
        //     p = val.next;
        // }

        // if let Some(tail) = tail {
        //     tail.next = Some(component);
        // } else {
        //     self.first_component = Some(component);
        // }

        // component.owner = Some(self.clone());
        // component.next = None;
        // component.on_added();

        // self
    }

    /// Remove a component from this entity.
    /// @return Whether the component was removed.
    pub fn remove(&self, component: &Component) -> bool {
        let prev: Option<Component> = None;
        let p = &self.first_component;
        todo!("should get entity from storage");
        // while let Some(ref val) = p {
        //     let next = val.next;
        //     if val == component {
        //         // Splice out the component
        //         if let Some(ref prev) = prev {
        //             prev.owner = Some(self.clone());
        //             prev.next = next;
        //         } else {
        //             self.first_component = next;
        //         }

        //         // Remove it from the _compMap
        //         // #[cfg(not(feature = "html"))]
        //         // __delete__(_compMap, p.name);
        //         // #[cfg(feature = "html")]
        //         // js("delete")(_compMap[p.name]);
        //         // #end

        //         // Notify the component it was removed
        //         if val.flags.contains(Component::STARTED) {
        //             val.on_stop();
        //             val.flags = val.flags.remove(Component::STARTED);
        //         }
        //         val.on_removed();
        //         val.owner = None;
        //         val.next = None;
        //         return true;
        //     }
        //     prev = p;
        //     p = next;
        // }

        // false
    }

    // /// Gets a component of a given type from this entity.
    // #[cfg(any(feature = "display", feature = "dox"))]
    // pub fn get<A: Component>(componentClass: Class<A>) -> Option<A> {
    //     None
    // }

    // #else
    // macro pub fn get<A:Component> (self :Expr, componentClass :ExprOf<Class<A>>) ->ExprOf<A>
    // {
    //     let type = requireComponentType(componentClass);
    //     let name = macro $componentClass.NAME;
    //     return needSafeCast(type)
    //         ? macro Std.instance($self.getComponent($name), $componentClass)
    //         : macro $self._internal_unsafeCast($self.getComponent($name), $componentClass);
    // }
    // #end

    // /// Checks if this entity has a component of the given type.
    // #[cfg(any(feature = "display", feature = "dox"))]
    // pub fn has<A: Component>(componentClass: Class<A>) -> bool {
    //     false
    // }

    // #else
    // macro pub fn has<A> (self :Expr, componentClass :ExprOf<Class<A>>) ->ExprOf<bool>
    // {
    //     return macro $self.get($componentClass).is_some();
    // }
    // #end

    // /// Gets a component of a given type from this entity, or any of its parents. Searches upwards in
    // /// the hierarchy until the component is found, or returns None if not found.
    // #[cfg(any(feature = "display", feature = "dox"))]
    // pub fn get_from_parents<A: Component>(componentClass: Class<A>) -> Option<A> {
    //     None
    // }

    // #else
    // macro pub fn get_from_parents<A> (self :Expr, componentClass :ExprOf<Class<A>>) ->ExprOf<A>
    // {
    //     let type = requireComponentType(componentClass);
    //     let name = macro $componentClass.NAME;
    //     return needSafeCast(type)
    //         ? macro $self._internal_get_from_parents($name, $componentClass)
    //         : macro $self._internal_unsafeCast($self._internal_get_from_parents($name), $componentClass);
    // }
    // #end

    // /// Gets a component of a given type from this entity, or any of its children. Searches downwards
    // /// in a depth-first search until the component is found, or returns None if not found.
    // #[cfg(any(feature = "display", feature = "dox"))]
    // pub fn get_from_children<A: Component>(componentClass: Class<A>) -> Option<A> {
    //     None
    // }

    // #else
    // macro pub fn get_from_children<A> (self :Expr, componentClass :ExprOf<Class<A>>) ->ExprOf<A>
    // {
    //     let type = requireComponentType(componentClass);
    //     let name = macro $componentClass.NAME;
    //     return needSafeCast(type)
    //         ? macro $self._internal_get_from_children($name, $componentClass)
    //         : macro $self._internal_unsafeCast($self._internal_get_from_children($name), $componentClass);
    // }
    // #end

    /// Gets a component by name from this entity.
    // #[inline]
    // pub fn get_component(&self, name: String) -> Option<&Component> {
    //     self.components.get(&name)
    // }

    /// Adds a child to this entity.
    /// @param append Whether to add the entity to the end or beginning of the child list.
    /// @returns This instance, for chaining.
    //  append :bool=true
    pub fn add_child(&self, entity: Entity, append: bool, z_order: Option<i32>) -> &Self {
        todo!("should get entity from storage");
        // log::info!("entity.addChild: {}", z_order);
        // if let Some(ref parent) = entity.parent {
        //     parent.remove_child(&entity);
        // }
        // entity.parent = Some(self.clone());
        // log::info!("append");

        // if append {
        //     // Append it to the child list
        //     let tail = None;
        //     let p = self.first_child;
        //     while let Some(ref val) = p {
        //         tail = Some(val);
        //         p = val.next;
        //     }

        //     if let Some(tail) = tail {
        //         tail.next = Some(entity);
        //     } else {
        //         self.first_child = Some(entity);
        //     }
        // } else {
        //     // Prepend it to the child list
        //     entity.next = self.first_child;
        //     self.first_child = Some(entity);
        // }

        // if (z_order == null) {
        // 	if (tailChild != null) {
        // 		z_order = tailChild.z_order;
        // 	} else {
        // 		z_order = 0;
        // 	}
        // }

        // if (append) {

        // 	let tail = null;
        //     let p = firstChild;

        // 	while (p != null) {
        // 		tail = p;
        // 		p = p.next;
        // 	}

        // 	if (tail != null) {
        // 		if (z_order == null) {
        // 			z_order = tail.z_order;
        // 		}
        // 		//log::info!(tail.z_order);
        // 		if (tail.z_order <= z_order) {
        // 			tail.next = entity;
        // 			//tailChild = entity;
        // 			//log::info!("append");
        // 		} else {
        // 			let p = firstChild;
        // 			let pre : Entity = null;
        // 			while (p != null) {
        // 				if (p.z_order > z_order) {
        // 					//log::info!("insert");
        // 					if (pre != null) {

        // 						pre.next = entity;
        // 						entity.next = p;

        // 					} else {
        // 						entity.next = firstChild;
        // 						firstChild = entity;
        // 					}
        // 					break;
        // 				} else {
        // 					pre = p;
        // 					p = p.next;
        // 				}

        // 			}
        // 		}
        // 	} else {
        // 		log::info!("init");
        // 		firstChild = entity;
        // 		if (z_order == null) {
        // 			z_order = 0;
        // 		}
        // 	}
        // } else {
        // 	if (firstChild == null) {
        // 		z_order = 0;
        // 	} else {
        // 		z_order = firstChild.z_order - 1;
        // 	}
        // 	entity.next = firstChild;
        //     firstChild = entity;
        // }

        // entity.z_order = z_order;

        // return this;

        // self
    }

    pub fn set_zorder(&mut self, z: i32) {
        if self.z_order == z {
            return;
        } else {
            self.z_order = z;
            todo!();
            // self.parent.addChild(self, true, self.z_order);
        }
    }

    pub fn remove_child(&self, entity: &Entity) {
        let prev: Option<Entity> = None;
        let p = self.first_child;
        todo!("should get entity from storage");
        // while let Some(ref val) = p {
        //     let next = val.next;
        //     if val == entity {
        //         // Splice out the entity
        //         if let Some(prev) = prev {
        //             prev.next = next;
        //         } else {
        //             self.first_child = next;
        //         }
        //         val.parent = None;
        //         val.next = None;
        //         return;
        //     }
        //     prev = p;
        //     p = next;
        // }
    }

    /// Dispose all of this entity's children, without touching its own components or removing itself
    /// from its parent.
    pub fn dispose_children(&self) {
        while let Some(ref first_child) = self.first_child {
            todo!("should get entity from storage");
            // first_child.dispose();
        }
    }

    fn to_string_impl<S: Into<String>>(&self, indent: S) -> String {
        let output = String::new();
        let p = &self.first_component;
        todo!("should get entity from stroage");
        // while let Some(ref val) = p {
        //     output.push_str(val.name.as_str());
        //     if val.next.is_some() {
        //         output.push_str(", ");
        //     }
        //     p = val.next;
        // }
        // output.push_str("\n");

        // // Oof, doesn't support escaped unicode in string literals
        // let u2514 = char::from_u32(0x2514).unwrap_or('└').to_string(); // └
        // let u241c = char::from_u32(0x251c).unwrap_or('├').to_string(); // ├
        // let u2500 = char::from_u32(0x2500).unwrap_or('─').to_string(); // ─
        // let u2502 = char::from_u32(0x2502).unwrap_or('│').to_string(); // │

        // let p = self.first_child;
        // while let Some(ref val) = p {
        //     todo!("should get entity from storage")
        //     // let last = val.next.is_none();
        //     // output.push_str(
        //     //     format!("{}{}{}{} ", indent.into(), (if last { u2514 } else { u241c }), u2500, u2500).as_str(),
        //     // );
        //     // output.push_str(
        //     //     val.toStringImpl(format!("{}{}   ", indent.into(), if last { " " } else { u2502.as_str() }))
        //     //         .as_str(),
        //     // );
        //     // p = val.next;
        // }

        // output
    }

    // Semi-helper methods
    // #[cfg(not(feature = "displays"))]
    // #[inline] pub fn _internal_unsafeCast<A:Component> (component :Component, cl :Class<A>) ->A
    // {
    //     return component;
    // }

    // pub fn _internal_get_from_parents<A:Component> (name :String, safeCast :Class<A>) ->A
    // {
    //     let entity = this;
    //     do {
    //         let component = entity.getComponent(name);
    //         if (safeCast.is_some()) {
    //             component = Std.instance(component, safeCast);
    //         }
    //         if (component.is_some()) {
    //             return cast component;
    //         }
    //         entity = entity.parent;
    //     } while (entity.is_some());

    //     return None; // Not found
    // }

    // pub fn _internal_get_from_children<A:Component> (name :String, ?safeCast :Class<A>) ->A
    // {
    //     let component = getComponent(name);
    //     if (safeCast.is_some()) {
    //         component = Std.instance(component, safeCast);
    //     }
    //     if (component.is_some()) {
    //         return cast component;
    //     }

    //     let child = first_child;
    //     while (child.is_some()) {
    //         let component = child._internal_get_from_children(name, safeCast);
    //         if (component.is_some()) {
    //             return component;
    //         }

    //         child = child.next;
    //     }

    //     return None; // Not found
    // }
    // #end

    // #[cfg(feature = "macro")]
    // // Gets the ClassType from an expression, or aborts if it's not a component class
    // static fn requireComponentType (componentClass :Expr) ->ClassType
    // {
    //     let path = getClassName(componentClass);
    //     if (path.is_some()) {
    //         let type = Context.getType(path.join("."));
    //         match (type) {
    //         case TInst(ref,_):
    //             let cl = ref.get();
    //             if (Context.unify(type, Context.getType("d2::Component")) && cl.superClass.is_some()) {
    //                 return cl;
    //             }
    //         default:
    //         }
    //     }

    //     Context.error("Expected a struct that extends Component, got " + componentClass.toString(),
    //         componentClass.pos);
    //     return None;
    // }

    // // Gets a struct name from a given expression
    // static fn getClassName<A> (componentClass :Expr) ->Vec<String>
    // {
    //     match (componentClass.expr) {
    //     case EConst(CIdent(name)):
    //         return [name];
    //     case EField(expr, name):
    //         let path = getClassName(expr);
    //         if (path.is_some()) {
    //             path.push(name);
    //         }
    //         return path;
    //     default:
    //         return None;
    //     }
    // }

    // static fn needSafeCast (componentClass :ClassType) ->bool
    // {
    //     return !componentClass.superClass.t.get().meta.has(":componentBase");
    // }
    // #end
}

impl AsRef<MoviePlayer> for Entity {
    fn as_ref(&self) -> &MoviePlayer {
        unimplemented!()
    }
}

impl AsRef<Sprite> for Entity {
    fn as_ref(&self) -> &Sprite {
        unimplemented!()
    }
}

impl<'a> AsRef<Option<&'a Sprite>> for Entity {
    fn as_ref(&self) -> &Option<&'a Sprite> {
        unimplemented!()
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_impl(""))
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        // self.path == other.path
        unimplemented!()
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        // self.path.partial_cmp(&other.path)
        unimplemented!()
    }
}

impl Disposable for Entity {
    /// Removes this entity from its parent, and disposes all its components and children.
    fn dispose(&self) {
        if let Some(parent) = self.parent {
            todo!("should get entity from storage");
            // parent.remove_child(self.id);
        }

        if let Some(ref first) = self.first_component {
            first.dispose();
        }

        self.dispose_children();
    }
}
