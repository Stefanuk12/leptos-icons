use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect height = "18" rx = "2" y = "4" x = "3" width = "18" ></ rect > < circle cy = "10" r = "2" cx = "12" ></ circle > < line x2 = "8" y1 = "2" y2 = "4" x1 = "8" ></ line > < line x2 = "16" x1 = "16" y1 = "2" y2 = "4" ></ line > < / > } } pub const LucideContact : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;