use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "9" r = "1" ></ circle > < circle cx = "9" r = "1" cy = "5" ></ circle > < circle cy = "19" r = "1" cx = "9" ></ circle > < circle r = "1" cy = "12" cx = "15" ></ circle > < circle cy = "5" r = "1" cx = "15" ></ circle > < circle cy = "19" r = "1" cx = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;