use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect y = "4" rx = "2" width = "18" height = "18" x = "3" ></ rect > < circle cx = "12" cy = "10" r = "2" ></ circle > < line y2 = "4" x1 = "8" x2 = "8" y1 = "2" ></ line > < line x1 = "16" x2 = "16" y1 = "2" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;