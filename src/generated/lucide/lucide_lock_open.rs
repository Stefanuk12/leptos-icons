use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" ry = "2" rx = "2" y = "11" width = "18" height = "11" ></ rect > < path d = "M7 11V7a5 5 0 0 1 9.9-1" ></ path > < / > } } pub const LUCIDE_LOCK_OPEN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;