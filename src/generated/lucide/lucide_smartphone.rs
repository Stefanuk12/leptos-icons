use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "20" rx = "2" y = "2" ry = "2" x = "5" ></ rect > < path d = "M12 18h.01" ></ path > < / > } } pub const LUCIDE_SMARTPHONE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2")] } ;