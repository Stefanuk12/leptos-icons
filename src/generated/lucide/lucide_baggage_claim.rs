use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2" ></ path > < path d = "M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10" ></ path > < rect rx = "1" x = "8" width = "13" height = "8" y = "6" ></ rect > < circle cx = "18" cy = "20" r = "2" ></ circle > < circle cx = "9" cy = "20" r = "2" ></ circle > < / > } } pub const LUCIDE_BAGGAGE_CLAIM : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;