use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "16" r = "1" ></ circle > < rect width = "18" rx = "2" x = "3" height = "12" y = "10" ></ rect > < path d = "M7 10V7a5 5 0 0 1 9.33-2.5" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE_OPEN : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;