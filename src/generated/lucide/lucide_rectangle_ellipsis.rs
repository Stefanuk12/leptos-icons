use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" x = "2" rx = "2" y = "6" width = "20" ></ rect > < path d = "M12 12h.01" ></ path > < path d = "M17 12h.01" ></ path > < path d = "M7 12h.01" ></ path > < / > } } pub const LUCIDE_RECTANGLE_ELLIPSIS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor")] } ;