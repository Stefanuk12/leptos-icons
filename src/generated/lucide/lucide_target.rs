use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < circle r = "6" cy = "12" cx = "12" ></ circle > < circle cx = "12" r = "2" cy = "12" ></ circle > < / > } } pub const LUCIDE_TARGET : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;