use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 20a6 6 0 0 0-12 0" ></ path > < circle cy = "10" cx = "12" r = "4" ></ circle > < circle r = "10" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_USER_ROUND : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;