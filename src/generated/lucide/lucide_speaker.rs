use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" x = "4" rx = "2" height = "20" y = "2" ></ rect > < path d = "M12 6h.01" ></ path > < circle r = "4" cx = "12" cy = "14" ></ circle > < path d = "M12 14h.01" ></ path > < / > } } pub const LUCIDE_SPEAKER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;