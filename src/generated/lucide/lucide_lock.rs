use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" ry = "2" height = "11" x = "3" y = "11" ></ rect > < path d = "M7 11V7a5 5 0 0 1 10 0v4" ></ path > < / > } } pub const LUCIDE_LOCK : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;