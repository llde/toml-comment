pub use toml_comment_derive::TomlComment;

pub trait TomlComment: serde::Serialize {
    fn to_commented_toml(&self) -> String;

    #[doc(hidden)]
    fn _render(&self, out: &mut String, prefix: &str);
}

pub trait TomlCommentDefault: TomlComment + Default{
    fn default_toml() -> String;
}

impl <T> TomlCommentDefault for T where T : TomlComment + Default {
    fn default_toml() -> String {
        T::default().to_commented_toml()
    }
}

impl<T> TomlComment for &T 
where 
    T: TomlComment + ?Sized 
{
    fn to_commented_toml(&self) -> String{
        (*self).to_commented_toml()
    }
    #[doc(hidden)]
    fn _render(&self, out: &mut String, prefix: &str){
        (*self)._render(out,prefix)
    }

}

impl<T> TomlComment for &mut T 
where 
    T: TomlComment + ?Sized 
{
    fn to_commented_toml(&self) -> String{
        (**self).to_commented_toml()
    }
    #[doc(hidden)]
    fn _render(&self, out: &mut String, prefix: &str){
        (**self)._render(out,prefix)
    }
}

pub fn fmt_value(val: &toml::Value) -> String {
    match val {
        toml::Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => {
            let s = f.to_string();
            if s.contains('.') { s } else { format!("{s}.0") }
        }
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Array(arr) => {
            format!(
                "[{}]",
                arr.iter().map(fmt_value).collect::<Vec<_>>().join(", ")
            )
        }
        toml::Value::Table(t) => {
            let pairs = t.iter().map(|(k, v)| format!("{k} = {}", fmt_value(v)));
            format!("{{ {} }}", pairs.collect::<Vec<_>>().join(", "))
        }
        toml::Value::Datetime(dt) => dt.to_string(),
    }
}
