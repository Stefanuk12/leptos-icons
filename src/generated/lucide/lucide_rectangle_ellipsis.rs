use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" rx = "2" y = "6" height = "12" ></ rect > < path d = "M12 12h.01" ></ path > < path d = "M17 12h.01" ></ path > < path d = "M7 12h.01" ></ path > < / > } } pub const LUCIDE_RECTANGLE_ELLIPSIS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;