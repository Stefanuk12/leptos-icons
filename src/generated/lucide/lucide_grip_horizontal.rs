use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "9" cx = "12" r = "1" ></ circle > < circle cx = "19" cy = "9" r = "1" ></ circle > < circle cx = "5" cy = "9" r = "1" ></ circle > < circle cx = "12" r = "1" cy = "15" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle r = "1" cx = "5" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;