use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "13" rx = "1" y = "3" width = "7" x = "3" ></ rect > < path d = "m9 22 3-3 3 3" ></ path > < rect width = "7" x = "14" rx = "1" y = "3" height = "13" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_END : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;