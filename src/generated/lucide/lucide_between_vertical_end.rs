use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "13" width = "7" x = "3" y = "3" rx = "1" ></ rect > < path d = "m9 22 3-3 3 3" ></ path > < rect rx = "1" x = "14" y = "3" width = "7" height = "13" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_END : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;