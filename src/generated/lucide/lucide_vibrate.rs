use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 8 2 2-2 2 2 2-2 2" ></ path > < path d = "m22 8-2 2 2 2-2 2 2 2" ></ path > < rect rx = "1" height = "14" width = "8" x = "8" y = "5" ></ rect > < / > } } pub const LUCIDE_VIBRATE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor")] } ;