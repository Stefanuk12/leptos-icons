use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "14" height = "20" x = "5" y = "2" rx = "2" ></ rect > < path d = "M12 18h.01" ></ path > < / > } } pub const LUCIDE_SMARTPHONE : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;