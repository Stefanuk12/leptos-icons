use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect x = "3" width = "18" height = "18" rx = "2" y = "4" ></ rect > < circle cx = "12" r = "2" cy = "10" ></ circle > < line x1 = "8" x2 = "8" y2 = "4" y1 = "2" ></ line > < line y2 = "4" x1 = "16" y1 = "2" x2 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor")] } ;