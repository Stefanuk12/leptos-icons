use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" height = "16" width = "20" y = "4" rx = "2" ></ rect > < path d = "M2 8h20" ></ path > < circle cy = "14" cx = "8" r = "2" ></ circle > < path d = "M8 12h8" ></ path > < circle r = "2" cx = "16" cy = "14" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;