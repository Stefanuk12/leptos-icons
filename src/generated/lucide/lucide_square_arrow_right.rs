use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" rx = "2" y = "3" width = "18" ></ rect > < path d = "M8 12h8" ></ path > < path d = "m12 16 4-4-4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_RIGHT : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;