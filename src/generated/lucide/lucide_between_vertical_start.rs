use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" y = "8" height = "13" x = "3" width = "7" ></ rect > < path d = "m15 2-3 3-3-3" ></ path > < rect height = "13" y = "8" width = "7" x = "14" rx = "1" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;