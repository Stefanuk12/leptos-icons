use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle r = "3" cy = "11" cx = "12" ></ circle > < rect x = "3" y = "4" width = "18" rx = "2" height = "18" ></ rect > < line y1 = "2" y2 = "4" x1 = "8" x2 = "8" ></ line > < line x1 = "16" y1 = "2" x2 = "16" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;