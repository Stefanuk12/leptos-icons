use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "18" rx = "2" width = "18" x = "3" ></ rect > < path d = "m16 8-8 8" ></ path > < path d = "M16 16H8V8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_DOWN_LEFT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;