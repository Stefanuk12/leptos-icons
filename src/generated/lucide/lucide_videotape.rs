use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" height = "16" width = "20" y = "4" ></ rect > < path d = "M2 8h20" ></ path > < circle cy = "14" r = "2" cx = "8" ></ circle > < path d = "M8 12h8" ></ path > < circle cx = "16" cy = "14" r = "2" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;