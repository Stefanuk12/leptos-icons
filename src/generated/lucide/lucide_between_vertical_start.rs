use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "8" width = "7" height = "13" rx = "1" x = "3" ></ rect > < path d = "m15 2-3 3-3-3" ></ path > < rect width = "7" x = "14" rx = "1" y = "8" height = "13" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_START : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;