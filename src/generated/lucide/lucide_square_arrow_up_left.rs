use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" y = "3" x = "3" height = "18" ></ rect > < path d = "M8 16V8h8" ></ path > < path d = "M16 16 8 8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;