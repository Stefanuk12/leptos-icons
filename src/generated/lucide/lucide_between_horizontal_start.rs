use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "8" width = "13" height = "7" y = "3" rx = "1" ></ rect > < path d = "m2 9 3 3-3 3" ></ path > < rect rx = "1" height = "7" width = "13" x = "8" y = "14" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24")] } ;