use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" height = "7" y = "3" width = "13" x = "8" ></ rect > < path d = "m2 9 3 3-3 3" ></ path > < rect y = "14" x = "8" width = "13" height = "7" rx = "1" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_START : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;