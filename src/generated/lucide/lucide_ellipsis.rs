use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "1" ></ circle > < circle r = "1" cy = "12" cx = "19" ></ circle > < circle r = "1" cx = "5" cy = "12" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;