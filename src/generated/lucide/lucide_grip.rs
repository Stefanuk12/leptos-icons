use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "5" cx = "12" ></ circle > < circle r = "1" cx = "19" cy = "5" ></ circle > < circle r = "1" cy = "5" cx = "5" ></ circle > < circle r = "1" cy = "12" cx = "12" ></ circle > < circle cx = "19" r = "1" cy = "12" ></ circle > < circle r = "1" cy = "12" cx = "5" ></ circle > < circle cy = "19" cx = "12" r = "1" ></ circle > < circle cy = "19" cx = "19" r = "1" ></ circle > < circle r = "1" cx = "5" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRIP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;