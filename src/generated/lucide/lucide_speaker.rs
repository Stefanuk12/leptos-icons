use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" width = "16" y = "2" height = "20" rx = "2" ></ rect > < path d = "M12 6h.01" ></ path > < circle r = "4" cy = "14" cx = "12" ></ circle > < path d = "M12 14h.01" ></ path > < / > } } pub const LUCIDE_SPEAKER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;