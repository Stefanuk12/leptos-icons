use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "7" height = "13" y = "8" rx = "1" ></ rect > < path d = "m15 2-3 3-3-3" ></ path > < rect x = "14" width = "7" height = "13" y = "8" rx = "1" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_START : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24")] } ;