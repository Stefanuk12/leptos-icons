use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2" ></ path > < path d = "M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10" ></ path > < rect x = "8" height = "8" y = "6" width = "13" rx = "1" ></ rect > < circle cy = "20" cx = "18" r = "2" ></ circle > < circle cy = "20" cx = "9" r = "2" ></ circle > < / > } } pub const LUCIDE_BAGGAGE_CLAIM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;