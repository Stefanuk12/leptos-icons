use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "9" cy = "12" ></ circle > < circle cy = "5" cx = "9" r = "1" ></ circle > < circle cx = "9" cy = "19" r = "1" ></ circle > < circle cy = "12" r = "1" cx = "15" ></ circle > < circle cx = "15" cy = "5" r = "1" ></ circle > < circle cx = "15" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;