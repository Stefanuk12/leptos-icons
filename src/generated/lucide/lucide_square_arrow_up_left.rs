use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" x = "3" width = "18" height = "18" ></ rect > < path d = "M8 16V8h8" ></ path > < path d = "M16 16 8 8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;