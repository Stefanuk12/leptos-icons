use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "16" ></ circle > < rect x = "3" height = "12" y = "10" rx = "2" width = "18" ></ rect > < path d = "M7 10V7a5 5 0 0 1 10 0v3" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24")] } ;