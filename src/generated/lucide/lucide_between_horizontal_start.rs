use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "13" height = "7" x = "8" y = "3" rx = "1" ></ rect > < path d = "m2 9 3 3-3 3" ></ path > < rect x = "8" height = "7" y = "14" width = "13" rx = "1" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;