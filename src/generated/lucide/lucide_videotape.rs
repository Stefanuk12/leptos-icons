use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 8h20" ></ path > < circle r = "2" cx = "8" cy = "14" ></ circle > < path d = "M8 12h8" ></ path > < circle cx = "16" cy = "14" r = "2" ></ circle > < / > } } pub const LucideVideotape : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;