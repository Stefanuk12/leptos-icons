use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "13" x = "3" rx = "1" y = "3" ></ rect > < path d = "m9 22 3-3 3 3" ></ path > < rect x = "14" width = "7" y = "3" rx = "1" height = "13" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_END : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;