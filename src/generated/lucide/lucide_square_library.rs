use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" rx = "2" height = "18" x = "3" ></ rect > < path d = "M7 7v10" ></ path > < path d = "M11 7v10" ></ path > < path d = "m15 7 2 10" ></ path > < / > } } pub const LUCIDE_SQUARE_LIBRARY : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;