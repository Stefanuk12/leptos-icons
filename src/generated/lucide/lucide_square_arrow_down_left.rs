use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" rx = "2" width = "18" x = "3" ></ rect > < path d = "m16 8-8 8" ></ path > < path d = "M16 16H8V8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_DOWN_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;