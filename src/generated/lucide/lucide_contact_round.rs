use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle r = "3" cy = "11" cx = "12" ></ circle > < rect x = "3" height = "18" y = "4" rx = "2" width = "18" ></ rect > < line x1 = "8" x2 = "8" y1 = "2" y2 = "4" ></ line > < line y2 = "4" y1 = "2" x2 = "16" x1 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24")] } ;