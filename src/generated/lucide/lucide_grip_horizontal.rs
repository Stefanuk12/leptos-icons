use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "9" ></ circle > < circle cy = "9" cx = "19" r = "1" ></ circle > < circle r = "1" cy = "9" cx = "5" ></ circle > < circle cx = "12" r = "1" cy = "15" ></ circle > < circle cx = "19" r = "1" cy = "15" ></ circle > < circle cx = "5" r = "1" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;