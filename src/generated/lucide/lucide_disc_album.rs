use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" width = "18" y = "3" rx = "2" ></ rect > < circle cx = "12" cy = "12" r = "5" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LucideDiscAlbum : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;