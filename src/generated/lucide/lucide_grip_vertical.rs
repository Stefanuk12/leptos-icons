use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" r = "1" cy = "12" ></ circle > < circle r = "1" cx = "9" cy = "5" ></ circle > < circle r = "1" cx = "9" cy = "19" ></ circle > < circle cx = "15" cy = "12" r = "1" ></ circle > < circle r = "1" cx = "15" cy = "5" ></ circle > < circle cx = "15" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;