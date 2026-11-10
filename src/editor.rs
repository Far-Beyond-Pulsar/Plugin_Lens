//! What the lens plugin registers for the editor (the `editor` feature): the
//! inspector's editor of a lens's element stack.

// The element stack edits as a reflected list (one card per element).
ui_common::register_reflected_list_editor!(lens_component::LensElementProps);

#[cfg(test)]
mod tests {
    /// The element stack has the inspector's list editor.
    #[test]
    fn element_lists_have_a_registered_editor() {
        let id = std::any::TypeId::of::<Vec<lens_component::LensElementProps>>();
        assert!(ui_common::PROPERTY_EDITOR_REGISTRY.has(id));
    }
}
