use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" y = "3" x = "3" height = "18" ></ rect > < path d = "M7 7v10" ></ path > < path d = "M11 7v10" ></ path > < path d = "m15 7 2 10" ></ path > < / > } } pub const LUCIDE_SQUARE_LIBRARY : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;