use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 11 2-2-2-2" ></ path > < path d = "M11 13h4" ></ path > < rect height = "18" ry = "2" y = "3" width = "18" x = "3" rx = "2" ></ rect > < / > } } pub const LUCIDE_SQUARE_TERMINAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;