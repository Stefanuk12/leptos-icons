use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 8 2 2-2 2 2 2-2 2" ></ path > < path d = "m22 8-2 2 2 2-2 2 2 2" ></ path > < rect width = "8" y = "5" rx = "1" x = "8" height = "14" ></ rect > < / > } } pub const LUCIDE_VIBRATE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;