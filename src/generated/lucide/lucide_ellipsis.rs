use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "12" ></ circle > < circle cy = "12" cx = "19" r = "1" ></ circle > < circle cx = "5" r = "1" cy = "12" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;