use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" height = "18" width = "18" rx = "2" ></ rect > < path d = "M12 8v8" ></ path > < path d = "m8 12 4 4 4-4" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_DOWN : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;