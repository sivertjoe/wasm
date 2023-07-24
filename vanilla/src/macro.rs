pub fn document() -> web_sys::Document {
    let document = web_sys::window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    document
}

#[macro_export]
macro_rules! attrs {
    ($($name:expr => $value:expr),* $(,)?) => {{
        $(Attr {name: $name.to_string(), value: $value.to_string()}),*

    }}
}

#[macro_export]
macro_rules! styles {
    ($($name:expr => $value:expr),* $(,)?) => {{
        let s = [$(format!("{}:{}", $name, $value)),*].join(";");
        Attr {
            name: "style".to_string(),
            value: s
        }
    }}
}

#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

#[macro_export]
macro_rules! define_elements {
    ($($Tag:ident),+) => {
        with_dollar_sign! {
            ($d:tt) => {
                $(
                    #[macro_export]
                    macro_rules! $Tag {
                        ( $d($d part:expr),* $d(,)? ) => {
                            {
                                let elem = document().create_element(stringify!($Tag)).unwrap();

                                $d(
                                     $d part.update_elem(&elem);
                                )*

                                elem
                            }
                        };
                    }
                )+
            }
        }
   }
}

define_elements! {
    div, h1, h2
}
