use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" rx = "2" width = "20" height = "16" x = "2" ></ rect > < path d = "M2 8h20" ></ path > < circle cy = "14" cx = "8" r = "2" ></ circle > < path d = "M8 12h8" ></ path > < circle cx = "16" cy = "14" r = "2" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;