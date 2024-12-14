use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "tagging_xmp.stpl")]
struct XmpTemplate<'x> {
    tags: Vec<&'x str>,
}

pub fn build_xmp(tag_block: &str) -> String {
    let temp = XmpTemplate {
        tags: tag_block.split(" ").collect(),
    };

    temp.render_once().unwrap()
}
