use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "1" height = "7" width = "13" ></ rect > < path d = "m22 15-3-3 3-3" ></ path > < rect x = "3" height = "7" y = "14" width = "13" rx = "1" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;