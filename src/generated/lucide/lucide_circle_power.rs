use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < path d = "M12 12V7" ></ path > < path d = "M16 9a5 5 0 1 1-8 0" ></ path > < / > } } pub const LUCIDE_CIRCLE_POWER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;