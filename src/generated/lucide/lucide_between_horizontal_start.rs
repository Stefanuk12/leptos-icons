use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" x = "8" height = "7" width = "13" y = "3" ></ rect > < path d = "m2 9 3 3-3 3" ></ path > < rect height = "7" x = "8" y = "14" rx = "1" width = "13" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;