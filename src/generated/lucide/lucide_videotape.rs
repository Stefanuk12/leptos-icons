use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" height = "16" x = "2" rx = "2" width = "20" ></ rect > < path d = "M2 8h20" ></ path > < circle cy = "14" r = "2" cx = "8" ></ circle > < path d = "M8 12h8" ></ path > < circle cy = "14" r = "2" cx = "16" ></ circle > < / > } } pub const LUCIDE_VIDEOTAPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;