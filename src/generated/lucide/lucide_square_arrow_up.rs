use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" height = "18" rx = "2" width = "18" ></ rect > < path d = "m16 12-4-4-4 4" ></ path > < path d = "M12 16V8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;