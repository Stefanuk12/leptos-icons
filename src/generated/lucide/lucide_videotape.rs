use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" width = "20" rx = "2" height = "16" x = "2" ></ rect > < path d = "M2 8h20" ></ path > < circle cy = "14" cx = "8" r = "2" ></ circle > < path d = "M8 12h8" ></ path > < circle r = "2" cy = "14" cx = "16" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor")] } ;