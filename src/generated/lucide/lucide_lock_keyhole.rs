use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "16" ></ circle > < rect y = "10" x = "3" rx = "2" width = "18" height = "12" ></ rect > < path d = "M7 10V7a5 5 0 0 1 10 0v3" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;