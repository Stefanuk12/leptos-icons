use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "9" r = "1" cx = "12" ></ circle > < circle r = "1" cx = "19" cy = "9" ></ circle > < circle cx = "5" cy = "9" r = "1" ></ circle > < circle r = "1" cx = "12" cy = "15" ></ circle > < circle r = "1" cy = "15" cx = "19" ></ circle > < circle r = "1" cy = "15" cx = "5" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;