use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect rx = "2" height = "18" x = "3" width = "18" y = "4" ></ rect > < circle cx = "12" cy = "10" r = "2" ></ circle > < line x1 = "8" y1 = "2" x2 = "8" y2 = "4" ></ line > < line y2 = "4" y1 = "2" x1 = "16" x2 = "16" ></ line > < / > } } pub const LucideContact : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;