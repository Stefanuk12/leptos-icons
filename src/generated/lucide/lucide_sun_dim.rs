use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "4" ></ circle > < path d = "M12 4h.01" ></ path > < path d = "M20 12h.01" ></ path > < path d = "M12 20h.01" ></ path > < path d = "M4 12h.01" ></ path > < path d = "M17.657 6.343h.01" ></ path > < path d = "M17.657 17.657h.01" ></ path > < path d = "M6.343 17.657h.01" ></ path > < path d = "M6.343 6.343h.01" ></ path > < / > } } pub const LUCIDE_SUN_DIM : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;