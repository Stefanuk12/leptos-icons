use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "1" cx = "12" ></ circle > < circle cx = "12" cy = "5" r = "1" ></ circle > < circle r = "1" cy = "19" cx = "12" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2")] } ;