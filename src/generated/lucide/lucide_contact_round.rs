use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cx = "12" cy = "11" r = "3" ></ circle > < rect y = "4" rx = "2" height = "18" width = "18" x = "3" ></ rect > < line x2 = "8" y2 = "4" y1 = "2" x1 = "8" ></ line > < line y1 = "2" x1 = "16" x2 = "16" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2")] } ;