use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" height = "18" x = "3" width = "18" ></ rect > < path d = "m12 8-4 4 4 4" ></ path > < path d = "M16 12H8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_LEFT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;