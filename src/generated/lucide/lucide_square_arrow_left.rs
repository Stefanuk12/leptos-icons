use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" rx = "2" height = "18" x = "3" ></ rect > < path d = "m12 8-4 4 4 4" ></ path > < path d = "M16 12H8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;