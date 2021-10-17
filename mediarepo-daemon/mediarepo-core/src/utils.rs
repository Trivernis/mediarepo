/// Parses a normalized tag into its two components of namespace and tag
pub fn parse_namespace_and_tag(norm_tag: String) -> (Option<String>, String) {
    norm_tag
        .split_once(':')
        .map(|(n, t)| (Some(n.trim().to_string()), t.trim().to_string()))
        .unwrap_or((None, norm_tag.trim().to_string()))
}
