use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" x = "3" width = "13" rx = "1" ></ rect > < path d = "m22 15-3-3 3-3" ></ path > < rect width = "13" height = "7" y = "14" rx = "1" x = "3" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;