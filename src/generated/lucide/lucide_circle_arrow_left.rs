use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < path d = "M16 12H8" ></ path > < path d = "m12 8-4 4 4 4" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;