use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "1" cx = "12" ></ circle > < circle cy = "5" r = "1" cx = "19" ></ circle > < circle cx = "5" r = "1" cy = "5" ></ circle > < circle r = "1" cy = "12" cx = "12" ></ circle > < circle cy = "12" r = "1" cx = "19" ></ circle > < circle r = "1" cy = "12" cx = "5" ></ circle > < circle r = "1" cy = "19" cx = "12" ></ circle > < circle r = "1" cy = "19" cx = "19" ></ circle > < circle cx = "5" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24")] } ;