use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect y = "4" x = "3" width = "18" rx = "2" height = "18" ></ rect > < circle r = "2" cx = "12" cy = "10" ></ circle > < line y1 = "2" y2 = "4" x1 = "8" x2 = "8" ></ line > < line x2 = "16" y1 = "2" y2 = "4" x1 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;