use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" ry = "2" rx = "2" y = "2" x = "5" width = "14" ></ rect > < path d = "M12 18h.01" ></ path > < / > } } pub const LUCIDE_SMARTPHONE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;