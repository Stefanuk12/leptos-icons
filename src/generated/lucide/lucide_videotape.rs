use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" y = "4" width = "20" rx = "2" height = "16" ></ rect > < path d = "M2 8h20" ></ path > < circle cy = "14" r = "2" cx = "8" ></ circle > < path d = "M8 12h8" ></ path > < circle cx = "16" cy = "14" r = "2" ></ circle > < / > } } pub const LucideVideotape : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;