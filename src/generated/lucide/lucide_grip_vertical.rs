use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "1" cx = "9" ></ circle > < circle cx = "9" r = "1" cy = "5" ></ circle > < circle r = "1" cx = "9" cy = "19" ></ circle > < circle cy = "12" cx = "15" r = "1" ></ circle > < circle cy = "5" r = "1" cx = "15" ></ circle > < circle cy = "19" r = "1" cx = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;