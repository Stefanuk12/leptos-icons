use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 9 5 12 1.8-5.2L21 14Z" ></ path > < path d = "M7.2 2.2 8 5.1" ></ path > < path d = "m5.1 8-2.9-.8" ></ path > < path d = "M14 4.1 12 6" ></ path > < path d = "m6 12-1.9 2" ></ path > < / > } } pub const LUCIDE_MOUSE_POINTER_CLICK : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;