use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "1" cx = "12" ></ circle > < circle cx = "12" cy = "5" r = "1" ></ circle > < circle cx = "12" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;