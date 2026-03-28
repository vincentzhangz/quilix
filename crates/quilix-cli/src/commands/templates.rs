use crate::commands::utils::{to_kebab_case, to_pascal_case};

fn generate_page_template(name: &str) -> String {
    format!(
        r#"export default function {}Page() {{
  return (
    <main>
      <h1>{}</h1>
    </main>
  );
}}
"#,
        to_pascal_case(name),
        to_pascal_case(name)
    )
}

fn generate_component_template(name: &str, kebab: &str) -> String {
    format!(
        r#"interface {}Props {{
  children?: React.ReactNode;
}}

export function {}Component({{ children }}: {}Props) {{
  return (
    <div className="{}">
      {{children}}
    </div>
  );
}}
"#,
        to_pascal_case(name),
        to_pascal_case(name),
        to_pascal_case(name),
        kebab
    )
}

fn generate_api_template(name: &str) -> String {
    format!(
        r#"export async function GET() {{
  return new Response(
    JSON.stringify({{
      message: 'Hello from {} API',
    }}),
    {{
      status: 200,
      headers: {{ 'Content-Type': 'application/json' }},
    }}
  );
}}
"#,
        name.replace('-', "_")
    )
}

pub(crate) fn get_page_template(name: &str) -> String {
    generate_page_template(name)
}

pub(crate) fn get_component_template(name: &str) -> String {
    generate_component_template(name, &to_kebab_case(name))
}

pub(crate) fn get_api_template(name: &str) -> String {
    generate_api_template(name)
}
