use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "9" ></ circle > < circle cy = "9" r = "1" cx = "19" ></ circle > < circle cx = "5" cy = "9" r = "1" ></ circle > < circle r = "1" cx = "12" cy = "15" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle cy = "15" cx = "5" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;