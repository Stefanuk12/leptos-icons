use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" width = "18" height = "18" x = "3" ></ rect > < path d = "M8 16V8h8" ></ path > < path d = "M16 16 8 8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;