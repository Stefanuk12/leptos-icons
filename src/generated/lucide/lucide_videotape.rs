use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "16" x = "2" y = "4" rx = "2" ></ rect > < path d = "M2 8h20" ></ path > < circle cx = "8" r = "2" cy = "14" ></ circle > < path d = "M8 12h8" ></ path > < circle cx = "16" cy = "14" r = "2" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;