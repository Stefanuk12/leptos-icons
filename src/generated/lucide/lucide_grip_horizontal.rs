use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "9" r = "1" cx = "12" ></ circle > < circle r = "1" cx = "19" cy = "9" ></ circle > < circle cy = "9" r = "1" cx = "5" ></ circle > < circle cx = "12" cy = "15" r = "1" ></ circle > < circle r = "1" cx = "19" cy = "15" ></ circle > < circle cx = "5" r = "1" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;