use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" height = "18" rx = "2" width = "18" ></ rect > < path d = "m12 8-4 4 4 4" ></ path > < path d = "M16 12H8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;