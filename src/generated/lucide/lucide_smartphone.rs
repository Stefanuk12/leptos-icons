use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" x = "5" rx = "2" y = "2" width = "14" ry = "2" ></ rect > < path d = "M12 18h.01" ></ path > < / > } } pub const LUCIDE_SMARTPHONE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;