use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cx = "12" cy = "11" r = "3" ></ circle > < rect height = "18" y = "4" width = "18" x = "3" rx = "2" ></ rect > < line x1 = "8" y2 = "4" x2 = "8" y1 = "2" ></ line > < line x1 = "16" x2 = "16" y2 = "4" y1 = "2" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;