use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cy = "11" cx = "12" r = "3" ></ circle > < rect x = "3" height = "18" width = "18" rx = "2" y = "4" ></ rect > < line y2 = "4" x1 = "8" y1 = "2" x2 = "8" ></ line > < line x2 = "16" x1 = "16" y1 = "2" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;