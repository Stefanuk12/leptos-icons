use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "16" r = "1" cx = "12" ></ circle > < rect height = "12" x = "3" y = "10" width = "18" rx = "2" ></ rect > < path d = "M7 10V7a5 5 0 0 1 9.33-2.5" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE_OPEN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;