use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "16" ></ circle > < rect width = "18" rx = "2" y = "10" x = "3" height = "12" ></ rect > < path d = "M7 10V7a5 5 0 0 1 9.33-2.5" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE_OPEN : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;