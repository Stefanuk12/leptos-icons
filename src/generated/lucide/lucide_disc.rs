use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < circle cx = "12" cy = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_DISC : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;