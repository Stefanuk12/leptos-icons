use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "9" r = "1" ></ circle > < circle cy = "5" cx = "9" r = "1" ></ circle > < circle r = "1" cy = "19" cx = "9" ></ circle > < circle cy = "12" r = "1" cx = "15" ></ circle > < circle cx = "15" cy = "5" r = "1" ></ circle > < circle r = "1" cy = "19" cx = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;