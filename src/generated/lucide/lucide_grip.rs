use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "5" r = "1" ></ circle > < circle cy = "5" r = "1" cx = "19" ></ circle > < circle r = "1" cy = "5" cx = "5" ></ circle > < circle cy = "12" r = "1" cx = "12" ></ circle > < circle cx = "19" r = "1" cy = "12" ></ circle > < circle cy = "12" r = "1" cx = "5" ></ circle > < circle r = "1" cx = "12" cy = "19" ></ circle > < circle cx = "19" cy = "19" r = "1" ></ circle > < circle r = "1" cx = "5" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRIP : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;