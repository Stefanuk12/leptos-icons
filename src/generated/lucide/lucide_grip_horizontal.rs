use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "9" ></ circle > < circle cx = "19" r = "1" cy = "9" ></ circle > < circle r = "1" cx = "5" cy = "9" ></ circle > < circle cx = "12" cy = "15" r = "1" ></ circle > < circle cy = "15" r = "1" cx = "19" ></ circle > < circle cy = "15" cx = "5" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;