use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 20a6 6 0 0 0-12 0" ></ path > < circle r = "4" cx = "12" cy = "10" ></ circle > < circle r = "10" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_USER_ROUND : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;