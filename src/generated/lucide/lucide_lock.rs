use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "18" height = "11" x = "3" rx = "2" y = "11" ></ rect > < path d = "M7 11V7a5 5 0 0 1 10 0v4" ></ path > < / > } } pub const LUCIDE_LOCK : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24")] } ;