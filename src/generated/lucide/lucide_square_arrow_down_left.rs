use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" height = "18" width = "18" ></ rect > < path d = "m16 8-8 8" ></ path > < path d = "M16 16H8V8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_DOWN_LEFT : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;