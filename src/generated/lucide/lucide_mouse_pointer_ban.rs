use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 2 4 11 2-5 5-2Z" ></ path > < circle r = "6" cx = "16" cy = "16" ></ circle > < path d = "m11.8 11.8 8.4 8.4" ></ path > < / > } } pub const LUCIDE_MOUSE_POINTER_BAN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;