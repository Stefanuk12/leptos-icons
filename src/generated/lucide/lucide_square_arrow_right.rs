use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "2" width = "18" height = "18" ></ rect > < path d = "M8 12h8" ></ path > < path d = "m12 16 4-4-4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_RIGHT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24")] } ;