use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "9" cx = "12" r = "1" ></ circle > < circle r = "1" cx = "19" cy = "9" ></ circle > < circle cx = "5" cy = "9" r = "1" ></ circle > < circle cy = "15" cx = "12" r = "1" ></ circle > < circle r = "1" cx = "19" cy = "15" ></ circle > < circle cy = "15" r = "1" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;