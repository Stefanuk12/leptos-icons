use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect width = "18" rx = "2" height = "18" y = "4" x = "3" ></ rect > < circle cx = "12" r = "2" cy = "10" ></ circle > < line y2 = "4" y1 = "2" x1 = "8" x2 = "8" ></ line > < line x2 = "16" x1 = "16" y2 = "4" y1 = "2" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;