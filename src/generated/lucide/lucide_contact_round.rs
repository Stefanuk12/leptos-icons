use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle r = "3" cx = "12" cy = "11" ></ circle > < rect width = "18" x = "3" y = "4" rx = "2" height = "18" ></ rect > < line x1 = "8" x2 = "8" y2 = "4" y1 = "2" ></ line > < line x1 = "16" y2 = "4" x2 = "16" y1 = "2" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;