use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "12" cx = "9" ></ circle > < circle cx = "9" cy = "5" r = "1" ></ circle > < circle cy = "19" cx = "9" r = "1" ></ circle > < circle r = "1" cy = "12" cx = "15" ></ circle > < circle r = "1" cy = "5" cx = "15" ></ circle > < circle cy = "19" r = "1" cx = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;