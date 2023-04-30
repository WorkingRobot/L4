fn main() {
    build_scripts::blueprint_compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "epic.gresource",
    );
}
