use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M288 96c0-17.7 14.3-32 32-32s32 14.3 32 32s14.3 32 32 32s32-14.3 32-32c0-53-43-96-96-96s-96 43-96 96l0 192-64 0 0-24c0-30.9-25.1-56-56-56l-48 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l48 0c4.4 0 8 3.6 8 8l0 24-80 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l224 0 224 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-80 0 0-24c0-4.4 3.6-8 8-8l56 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-56 0c-30.9 0-56 25.1-56 56l0 24-64 0 0-192zM480 416l0-32L32 384l0 32c0 53 43 96 96 96l256 0c53 0 96-43 96-96z" ></ path > < / > } } pub const FONT_AWESOME_SOLID_SINK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 512 512")] } ;