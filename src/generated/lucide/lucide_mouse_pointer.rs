use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 3 7.07 16.97 2.51-7.39 7.39-2.51L3 3z" ></ path > < path d = "m13 13 6 6" ></ path > < / > } } pub const LUCIDE_MOUSE_POINTER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24")] } ;