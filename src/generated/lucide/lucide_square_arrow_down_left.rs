use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" rx = "2" y = "3" x = "3" ></ rect > < path d = "m16 8-8 8" ></ path > < path d = "M16 16H8V8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_DOWN_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;