use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "9" ></ circle > < circle r = "1" cy = "9" cx = "19" ></ circle > < circle cy = "9" cx = "5" r = "1" ></ circle > < circle cy = "15" cx = "12" r = "1" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle r = "1" cx = "5" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;