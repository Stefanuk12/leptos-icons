use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 2 4 11 2-5 5-2Z" ></ path > < circle cx = "16" r = "6" cy = "16" ></ circle > < path d = "m11.8 11.8 8.4 8.4" ></ path > < / > } } pub const LUCIDE_MOUSE_POINTER_BAN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;