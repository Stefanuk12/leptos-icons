use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "1" cx = "12" ></ circle > < circle cy = "5" cx = "12" r = "1" ></ circle > < circle cx = "12" r = "1" cy = "19" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor")] } ;