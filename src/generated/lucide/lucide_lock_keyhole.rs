use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "16" cx = "12" ></ circle > < rect height = "12" width = "18" rx = "2" x = "3" y = "10" ></ rect > < path d = "M7 10V7a5 5 0 0 1 10 0v3" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24")] } ;