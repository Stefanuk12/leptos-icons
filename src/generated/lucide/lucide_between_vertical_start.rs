use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" y = "8" x = "3" rx = "1" height = "13" ></ rect > < path d = "m15 2-3 3-3-3" ></ path > < rect width = "7" y = "8" rx = "1" x = "14" height = "13" ></ rect > < / > } } pub const LUCIDE_BETWEEN_VERTICAL_START : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24")] } ;