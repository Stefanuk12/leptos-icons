use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" width = "20" rx = "2" y = "4" x = "2" ></ rect > < path d = "M2 8h20" ></ path > < circle cx = "8" cy = "14" r = "2" ></ circle > < path d = "M8 12h8" ></ path > < circle cy = "14" r = "2" cx = "16" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;