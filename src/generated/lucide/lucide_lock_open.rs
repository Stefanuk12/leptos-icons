use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "11" ry = "2" rx = "2" width = "18" x = "3" y = "11" ></ rect > < path d = "M7 11V7a5 5 0 0 1 9.9-1" ></ path > < / > } } pub const LUCIDE_LOCK_OPEN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;