use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cy = "11" r = "3" cx = "12" ></ circle > < rect x = "3" height = "18" width = "18" rx = "2" y = "4" ></ rect > < line x1 = "8" y1 = "2" y2 = "4" x2 = "8" ></ line > < line y2 = "4" y1 = "2" x2 = "16" x1 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;