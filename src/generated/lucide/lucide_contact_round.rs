use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cy = "11" cx = "12" r = "3" ></ circle > < rect width = "18" x = "3" y = "4" rx = "2" height = "18" ></ rect > < line y1 = "2" y2 = "4" x2 = "8" x1 = "8" ></ line > < line y2 = "4" x1 = "16" x2 = "16" y1 = "2" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;