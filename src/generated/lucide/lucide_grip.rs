use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" cx = "12" r = "1" ></ circle > < circle r = "1" cx = "19" cy = "5" ></ circle > < circle cy = "5" r = "1" cx = "5" ></ circle > < circle cx = "12" cy = "12" r = "1" ></ circle > < circle cx = "19" cy = "12" r = "1" ></ circle > < circle cy = "12" cx = "5" r = "1" ></ circle > < circle r = "1" cy = "19" cx = "12" ></ circle > < circle cx = "19" cy = "19" r = "1" ></ circle > < circle cx = "5" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2")] } ;