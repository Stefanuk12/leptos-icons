use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cy = "11" r = "3" cx = "12" ></ circle > < rect rx = "2" width = "18" y = "4" x = "3" height = "18" ></ rect > < line x2 = "8" x1 = "8" y1 = "2" y2 = "4" ></ line > < line y2 = "4" y1 = "2" x1 = "16" x2 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24")] } ;