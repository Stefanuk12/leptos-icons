use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "9" r = "1" ></ circle > < circle cy = "9" r = "1" cx = "19" ></ circle > < circle cy = "9" cx = "5" r = "1" ></ circle > < circle cy = "15" r = "1" cx = "12" ></ circle > < circle cy = "15" cx = "19" r = "1" ></ circle > < circle r = "1" cy = "15" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;