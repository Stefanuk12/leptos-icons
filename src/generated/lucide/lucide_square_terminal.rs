use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 11 2-2-2-2" ></ path > < path d = "M11 13h4" ></ path > < rect x = "3" y = "3" ry = "2" rx = "2" width = "18" height = "18" ></ rect > < / > } } pub const LUCIDE_SQUARE_TERMINAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;