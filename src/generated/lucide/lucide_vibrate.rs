use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 8 2 2-2 2 2 2-2 2" ></ path > < path d = "m22 8-2 2 2 2-2 2 2 2" ></ path > < rect width = "8" height = "14" rx = "1" x = "8" y = "5" ></ rect > < / > } } pub const LUCIDE_VIBRATE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;