use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "9" ></ circle > < circle r = "1" cx = "19" cy = "9" ></ circle > < circle cx = "5" r = "1" cy = "9" ></ circle > < circle cx = "12" cy = "15" r = "1" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle cx = "5" cy = "15" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;