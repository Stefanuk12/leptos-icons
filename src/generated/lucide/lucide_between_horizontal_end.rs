use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "7" rx = "1" y = "3" width = "13" ></ rect > < path d = "m22 15-3-3 3-3" ></ path > < rect y = "14" rx = "1" height = "7" width = "13" x = "3" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none")] } ;