use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect rx = "2" width = "18" y = "4" height = "18" x = "3" ></ rect > < circle cx = "12" cy = "10" r = "2" ></ circle > < line y1 = "2" y2 = "4" x2 = "8" x1 = "8" ></ line > < line y1 = "2" x2 = "16" y2 = "4" x1 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor")] } ;