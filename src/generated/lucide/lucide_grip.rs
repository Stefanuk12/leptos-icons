use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "5" cx = "12" ></ circle > < circle cy = "5" r = "1" cx = "19" ></ circle > < circle cx = "5" cy = "5" r = "1" ></ circle > < circle r = "1" cy = "12" cx = "12" ></ circle > < circle r = "1" cx = "19" cy = "12" ></ circle > < circle cx = "5" cy = "12" r = "1" ></ circle > < circle cx = "12" cy = "19" r = "1" ></ circle > < circle cx = "19" cy = "19" r = "1" ></ circle > < circle r = "1" cy = "19" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRIP : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;