use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle cy = "11" cx = "12" r = "3" ></ circle > < rect height = "18" y = "4" x = "3" rx = "2" width = "18" ></ rect > < line x1 = "8" y1 = "2" y2 = "4" x2 = "8" ></ line > < line x1 = "16" y1 = "2" y2 = "4" x2 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;