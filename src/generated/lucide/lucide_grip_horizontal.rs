use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "9" r = "1" cx = "12" ></ circle > < circle r = "1" cx = "19" cy = "9" ></ circle > < circle cy = "9" cx = "5" r = "1" ></ circle > < circle cx = "12" cy = "15" r = "1" ></ circle > < circle r = "1" cx = "19" cy = "15" ></ circle > < circle r = "1" cx = "5" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;