use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "11" ry = "2" rx = "2" width = "18" y = "11" x = "3" ></ rect > < path d = "M7 11V7a5 5 0 0 1 10 0v4" ></ path > < / > } } pub const LUCIDE_LOCK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;