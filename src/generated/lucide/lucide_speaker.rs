use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" width = "16" rx = "2" height = "20" x = "4" ></ rect > < path d = "M12 6h.01" ></ path > < circle cx = "12" r = "4" cy = "14" ></ circle > < path d = "M12 14h.01" ></ path > < / > } } pub const LUCIDE_SPEAKER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;