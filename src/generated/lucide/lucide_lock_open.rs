use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "11" ry = "2" width = "18" rx = "2" height = "11" ></ rect > < path d = "M7 11V7a5 5 0 0 1 9.9-1" ></ path > < / > } } pub const LUCIDE_LOCK_OPEN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;