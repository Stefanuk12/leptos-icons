use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "16" cx = "12" r = "1" ></ circle > < rect width = "18" height = "12" rx = "2" y = "10" x = "3" ></ rect > < path d = "M7 10V7a5 5 0 0 1 10 0v3" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;