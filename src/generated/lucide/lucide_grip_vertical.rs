use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "9" cy = "12" ></ circle > < circle r = "1" cx = "9" cy = "5" ></ circle > < circle cx = "9" cy = "19" r = "1" ></ circle > < circle cy = "12" r = "1" cx = "15" ></ circle > < circle cy = "5" cx = "15" r = "1" ></ circle > < circle r = "1" cx = "15" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;