use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "9" r = "1" ></ circle > < circle cx = "19" cy = "9" r = "1" ></ circle > < circle cy = "9" cx = "5" r = "1" ></ circle > < circle cx = "12" r = "1" cy = "15" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle cx = "5" r = "1" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;