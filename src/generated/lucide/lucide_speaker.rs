use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" y = "2" height = "20" rx = "2" width = "16" ></ rect > < path d = "M12 6h.01" ></ path > < circle cy = "14" cx = "12" r = "4" ></ circle > < path d = "M12 14h.01" ></ path > < / > } } pub const LUCIDE_SPEAKER : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor")] } ;