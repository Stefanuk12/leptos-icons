use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "13" height = "7" x = "8" y = "3" rx = "1" ></ rect > < path d = "m2 9 3 3-3 3" ></ path > < rect height = "7" width = "13" rx = "1" x = "8" y = "14" ></ rect > < / > } } pub const LUCIDE_BETWEEN_HORIZONTAL_START : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;