use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "7" x = "3" height = "13" y = "3" ></ rect > < path d = "m9 22 3-3 3 3" ></ path > < rect width = "7" x = "14" height = "13" rx = "1" y = "3" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_END : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;