use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "9" ></ circle > < circle cx = "19" cy = "9" r = "1" ></ circle > < circle cy = "9" r = "1" cx = "5" ></ circle > < circle cy = "15" cx = "12" r = "1" ></ circle > < circle cy = "15" r = "1" cx = "19" ></ circle > < circle cy = "15" r = "1" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;