use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4 4 7.07 17 2.51-7.39L21 11.07z" ></ path > < / > } } pub const LUCIDE_MOUSE_POINTER_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;