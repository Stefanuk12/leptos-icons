use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" rx = "2" y = "4" height = "16" ></ rect > < path d = "M2 8h20" ></ path > < circle r = "2" cx = "8" cy = "14" ></ circle > < path d = "M8 12h8" ></ path > < circle cx = "16" r = "2" cy = "14" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;