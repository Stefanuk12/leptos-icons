use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "5" ></ circle > < circle cy = "5" cx = "19" r = "1" ></ circle > < circle cy = "5" r = "1" cx = "5" ></ circle > < circle cy = "12" cx = "12" r = "1" ></ circle > < circle cy = "12" cx = "19" r = "1" ></ circle > < circle cy = "12" r = "1" cx = "5" ></ circle > < circle cx = "12" r = "1" cy = "19" ></ circle > < circle r = "1" cy = "19" cx = "19" ></ circle > < circle r = "1" cy = "19" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRIP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;