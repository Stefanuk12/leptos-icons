use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 22H4a2 2 0 0 1-2-2V6" ></ path > < path d = "m22 13-1.296-1.296a2.41 2.41 0 0 0-3.408 0L11 18" ></ path > < circle cx = "12" cy = "8" r = "2" ></ circle > < rect width = "16" rx = "2" x = "6" y = "2" height = "16" ></ rect > < / > } } pub const LucideImages : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;