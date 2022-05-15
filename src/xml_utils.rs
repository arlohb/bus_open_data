pub fn get_child<'a>(node: &'a roxmltree::Node, name: &'a str) -> Option<roxmltree::Node<'a, 'a>> {
  node.children().find(|child| child.tag_name().name() == name)
}

pub fn get_child_text<'a>(node: &'a roxmltree::Node, name: &'a str) -> Option<String> {
  Some(
    get_child(node, name)?
      .text()?
      .trim()
      .to_string()
  )
}
