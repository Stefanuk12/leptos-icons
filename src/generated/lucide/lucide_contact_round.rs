use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cx = "12" cy = "11" r = "3" ></ circle > < rect x = "3" y = "4" width = "18" height = "18" rx = "2" ></ rect > < line x2 = "8" y1 = "2" y2 = "4" x1 = "8" ></ line > < line y1 = "2" y2 = "4" x2 = "16" x1 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;