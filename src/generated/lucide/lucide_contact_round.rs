use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < circle r = "3" cx = "12" cy = "11" ></ circle > < rect rx = "2" height = "18" x = "3" y = "4" width = "18" ></ rect > < line y1 = "2" y2 = "4" x2 = "8" x1 = "8" ></ line > < line x2 = "16" y1 = "2" y2 = "4" x1 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT_ROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;