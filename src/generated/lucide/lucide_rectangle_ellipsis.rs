use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" width = "20" rx = "2" x = "2" y = "6" ></ rect > < path d = "M12 12h.01" ></ path > < path d = "M17 12h.01" ></ path > < path d = "M7 12h.01" ></ path > < / > } } pub const LUCIDE_RECTANGLE_ELLIPSIS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;