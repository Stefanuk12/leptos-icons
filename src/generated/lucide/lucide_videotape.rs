use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" width = "20" x = "2" rx = "2" y = "4" ></ rect > < path d = "M2 8h20" ></ path > < circle cx = "8" r = "2" cy = "14" ></ circle > < path d = "M8 12h8" ></ path > < circle cy = "14" cx = "16" r = "2" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;