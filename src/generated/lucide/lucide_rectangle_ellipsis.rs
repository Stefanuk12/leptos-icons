use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "6" width = "20" height = "12" x = "2" ></ rect > < path d = "M12 12h.01" ></ path > < path d = "M17 12h.01" ></ path > < path d = "M7 12h.01" ></ path > < / > } } pub const LUCIDE_RECTANGLE_ELLIPSIS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;