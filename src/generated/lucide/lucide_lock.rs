use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "11" ry = "2" x = "3" height = "11" width = "18" rx = "2" ></ rect > < path d = "M7 11V7a5 5 0 0 1 10 0v4" ></ path > < / > } } pub const LUCIDE_LOCK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;