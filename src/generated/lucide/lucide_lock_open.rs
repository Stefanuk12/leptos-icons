use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "11" x = "3" y = "11" width = "18" ry = "2" rx = "2" ></ rect > < path d = "M7 11V7a5 5 0 0 1 9.9-1" ></ path > < / > } } pub const LUCIDE_LOCK_OPEN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;