use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "12" cx = "9" ></ circle > < circle cx = "9" cy = "5" r = "1" ></ circle > < circle cx = "9" r = "1" cy = "19" ></ circle > < circle cx = "15" cy = "12" r = "1" ></ circle > < circle cx = "15" r = "1" cy = "5" ></ circle > < circle cx = "15" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;