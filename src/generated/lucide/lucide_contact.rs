use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect width = "18" y = "4" x = "3" height = "18" rx = "2" ></ rect > < circle cy = "10" r = "2" cx = "12" ></ circle > < line x1 = "8" y1 = "2" x2 = "8" y2 = "4" ></ line > < line x1 = "16" y1 = "2" x2 = "16" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;