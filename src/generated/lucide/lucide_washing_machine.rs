use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h3" ></ path > < path d = "M17 6h.01" ></ path > < rect y = "2" height = "20" width = "18" x = "3" rx = "2" ></ rect > < circle cx = "12" r = "5" cy = "13" ></ circle > < path d = "M12 18a2.5 2.5 0 0 0 0-5 2.5 2.5 0 0 1 0-5" ></ path > < / > } } pub const LUCIDE_WASHING_MACHINE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24")] } ;