use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 11 2-2-2-2" ></ path > < path d = "M11 13h4" ></ path > < rect rx = "2" width = "18" x = "3" height = "18" y = "3" ry = "2" ></ rect > < / > } } pub const LUCIDE_SQUARE_TERMINAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;