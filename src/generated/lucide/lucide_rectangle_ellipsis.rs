use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "20" y = "6" height = "12" rx = "2" ></ rect > < path d = "M12 12h.01" ></ path > < path d = "M17 12h.01" ></ path > < path d = "M7 12h.01" ></ path > < / > } } pub const LUCIDE_RECTANGLE_ELLIPSIS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;