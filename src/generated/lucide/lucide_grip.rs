use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "1" cx = "12" ></ circle > < circle cx = "19" cy = "5" r = "1" ></ circle > < circle cx = "5" cy = "5" r = "1" ></ circle > < circle cy = "12" r = "1" cx = "12" ></ circle > < circle cy = "12" cx = "19" r = "1" ></ circle > < circle r = "1" cx = "5" cy = "12" ></ circle > < circle r = "1" cy = "19" cx = "12" ></ circle > < circle cy = "19" cx = "19" r = "1" ></ circle > < circle cy = "19" r = "1" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRIP : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;