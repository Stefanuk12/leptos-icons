use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cx = "12" cy = "11" r = "3" ></ circle > < rect width = "18" height = "18" x = "3" y = "4" rx = "2" ></ rect > < line y2 = "4" x1 = "8" x2 = "8" y1 = "2" ></ line > < line y2 = "4" x2 = "16" x1 = "16" y1 = "2" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;