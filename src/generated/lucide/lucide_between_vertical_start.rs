use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" y = "8" height = "13" x = "3" width = "7" ></ rect > < path d = "m15 2-3 3-3-3" ></ path > < rect width = "7" height = "13" x = "14" rx = "1" y = "8" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_START : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;