use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 12a4 4 0 0 0 0-8H6v8" ></ path > < path d = "M15 20a4 4 0 0 0 0-8H6v8Z" ></ path > < / > } } pub const LUCIDE_BOLD : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;