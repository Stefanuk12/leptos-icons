use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" y = "4" rx = "2" height = "16" x = "2" ></ rect > < path d = "M2 8h20" ></ path > < circle cx = "8" r = "2" cy = "14" ></ circle > < path d = "M8 12h8" ></ path > < circle r = "2" cy = "14" cx = "16" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;