use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" y = "3" rx = "2" width = "18" ></ rect > < path d = "M7 7v10" ></ path > < path d = "M11 7v10" ></ path > < path d = "m15 7 2 10" ></ path > < / > } } pub const LUCIDE_SQUARE_LIBRARY : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2")] } ;