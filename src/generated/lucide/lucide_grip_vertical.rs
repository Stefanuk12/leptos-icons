use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" cy = "12" r = "1" ></ circle > < circle cx = "9" cy = "5" r = "1" ></ circle > < circle r = "1" cx = "9" cy = "19" ></ circle > < circle cy = "12" r = "1" cx = "15" ></ circle > < circle cy = "5" r = "1" cx = "15" ></ circle > < circle cx = "15" r = "1" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;