use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect x = "3" width = "18" rx = "2" y = "4" height = "18" ></ rect > < circle cy = "10" cx = "12" r = "2" ></ circle > < line y2 = "4" x2 = "8" y1 = "2" x1 = "8" ></ line > < line y1 = "2" x2 = "16" x1 = "16" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24")] } ;