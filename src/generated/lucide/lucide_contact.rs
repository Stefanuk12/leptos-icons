use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect height = "18" rx = "2" y = "4" x = "3" width = "18" ></ rect > < circle cx = "12" cy = "10" r = "2" ></ circle > < line x1 = "8" y1 = "2" x2 = "8" y2 = "4" ></ line > < line x1 = "16" x2 = "16" y2 = "4" y1 = "2" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24")] } ;