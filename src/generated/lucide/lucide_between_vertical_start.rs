use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "7" height = "13" rx = "1" y = "8" ></ rect > < path d = "m15 2-3 3-3-3" ></ path > < rect height = "13" x = "14" width = "7" rx = "1" y = "8" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_START : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;