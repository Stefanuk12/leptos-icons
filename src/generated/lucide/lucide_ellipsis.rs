use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "12" ></ circle > < circle cx = "19" r = "1" cy = "12" ></ circle > < circle cx = "5" cy = "12" r = "1" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;