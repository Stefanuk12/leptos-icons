use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cx = "12" r = "3" cy = "11" ></ circle > < rect rx = "2" x = "3" width = "18" height = "18" y = "4" ></ rect > < line y2 = "4" y1 = "2" x2 = "8" x1 = "8" ></ line > < line y1 = "2" x2 = "16" x1 = "16" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24")] } ;