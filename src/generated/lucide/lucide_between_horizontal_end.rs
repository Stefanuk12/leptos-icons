use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "13" y = "3" height = "7" x = "3" rx = "1" ></ rect > < path d = "m22 15-3-3 3-3" ></ path > < rect rx = "1" x = "3" width = "13" y = "14" height = "7" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;