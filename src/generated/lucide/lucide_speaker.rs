use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" x = "4" width = "16" height = "20" rx = "2" ></ rect > < path d = "M12 6h.01" ></ path > < circle cy = "14" r = "4" cx = "12" ></ circle > < path d = "M12 14h.01" ></ path > < / > } } pub const LUCIDE_SPEAKER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24")] } ;