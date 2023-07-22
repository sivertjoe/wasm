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
                                use $crate::UpdateElem;
                                #[allow(unused_mut)]
                                let mut elem = $crate::Element {
                                    name: stringify!($Tag).to_string(),
                                    attrs: Vec::new(),
                                    children: Vec::new()
                                };

                                $d(
                                     $d part.update_elem(&mut elem);
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
