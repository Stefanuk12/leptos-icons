use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect rx = "2" width = "18" height = "18" y = "4" x = "3" ></ rect > < circle cx = "12" cy = "10" r = "2" ></ circle > < line x1 = "8" y1 = "2" y2 = "4" x2 = "8" ></ line > < line x2 = "16" x1 = "16" y1 = "2" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;