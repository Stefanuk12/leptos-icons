use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cy = "11" r = "3" cx = "12" ></ circle > < rect width = "18" rx = "2" y = "4" height = "18" x = "3" ></ rect > < line y1 = "2" x1 = "8" x2 = "8" y2 = "4" ></ line > < line y1 = "2" x1 = "16" y2 = "4" x2 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;