use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" y = "2" rx = "2" width = "16" x = "4" ></ rect > < path d = "M12 6h.01" ></ path > < circle cx = "12" cy = "14" r = "4" ></ circle > < path d = "M12 14h.01" ></ path > < / > } } pub const LUCIDE_SPEAKER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;