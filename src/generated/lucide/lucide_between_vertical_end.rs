use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "13" x = "3" rx = "1" width = "7" ></ rect > < path d = "m9 22 3-3 3 3" ></ path > < rect rx = "1" y = "3" x = "14" height = "13" width = "7" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_END : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none")] } ;