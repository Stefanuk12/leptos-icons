use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "9" ></ circle > < circle cy = "9" cx = "19" r = "1" ></ circle > < circle cy = "9" cx = "5" r = "1" ></ circle > < circle cx = "12" cy = "15" r = "1" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle cx = "5" cy = "15" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;