use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "16" ></ circle > < rect width = "18" height = "12" y = "10" rx = "2" x = "3" ></ rect > < path d = "M7 10V7a5 5 0 0 1 9.33-2.5" ></ path > < / > } } pub const LUCIDE_LOCK_KEYHOLE_OPEN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;