use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h3" ></ path > < path d = "M17 6h.01" ></ path > < rect x = "3" rx = "2" width = "18" y = "2" height = "20" ></ rect > < circle cy = "13" cx = "12" r = "5" ></ circle > < path d = "M12 18a2.5 2.5 0 0 0 0-5 2.5 2.5 0 0 1 0-5" ></ path > < / > } } pub const LUCIDE_WASHING_MACHINE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;