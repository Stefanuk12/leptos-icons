use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 8 2 2-2 2 2 2-2 2" ></ path > < path d = "m22 8-2 2 2 2-2 2 2 2" ></ path > < rect x = "8" height = "14" width = "8" y = "5" rx = "1" ></ rect > < / > } } pub const LUCIDE_VIBRATE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;