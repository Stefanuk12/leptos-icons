use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "11" x = "3" rx = "2" width = "18" height = "11" ry = "2" ></ rect > < path d = "M7 11V7a5 5 0 0 1 9.9-1" ></ path > < / > } } pub const LUCIDE_LOCK_OPEN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;