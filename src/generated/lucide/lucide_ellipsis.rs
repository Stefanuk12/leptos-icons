use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "1" ></ circle > < circle cy = "12" cx = "19" r = "1" ></ circle > < circle r = "1" cy = "12" cx = "5" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;