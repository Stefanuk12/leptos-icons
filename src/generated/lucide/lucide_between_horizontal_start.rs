use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "13" height = "7" x = "8" y = "3" ></ rect > < path d = "m2 9 3 3-3 3" ></ path > < rect x = "8" rx = "1" height = "7" width = "13" y = "14" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;