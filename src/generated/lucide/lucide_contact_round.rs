use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle r = "3" cx = "12" cy = "11" ></ circle > < rect y = "4" rx = "2" x = "3" width = "18" height = "18" ></ rect > < line y2 = "4" x2 = "8" x1 = "8" y1 = "2" ></ line > < line x1 = "16" x2 = "16" y1 = "2" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round")] } ;