use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 11 2-2-2-2" ></ path > < path d = "M11 13h4" ></ path > < rect ry = "2" x = "3" width = "18" height = "18" rx = "2" y = "3" ></ rect > < / > } } pub const LUCIDE_SQUARE_TERMINAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;