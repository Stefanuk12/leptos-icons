use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect rx = "2" width = "18" height = "18" x = "3" y = "4" ></ rect > < circle cx = "12" cy = "10" r = "2" ></ circle > < line y1 = "2" y2 = "4" x1 = "8" x2 = "8" ></ line > < line x1 = "16" y1 = "2" y2 = "4" x2 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;