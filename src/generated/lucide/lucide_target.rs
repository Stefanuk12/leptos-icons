use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < circle r = "6" cy = "12" cx = "12" ></ circle > < circle cx = "12" r = "2" cy = "12" ></ circle > < / > } } pub const LUCIDE_TARGET : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;