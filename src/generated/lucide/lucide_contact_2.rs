use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cy = "11" r = "3" cx = "12" ></ circle > < rect x = "3" width = "18" height = "18" y = "4" rx = "2" ></ rect > < line x1 = "8" y2 = "4" x2 = "8" y1 = "2" ></ line > < line x1 = "16" y2 = "4" x2 = "16" y1 = "2" ></ line > < / > } } pub const LucideContact2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor")] } ;