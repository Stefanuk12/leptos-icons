use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" cy = "12" r = "1" ></ circle > < circle cx = "9" cy = "5" r = "1" ></ circle > < circle cx = "9" r = "1" cy = "19" ></ circle > < circle cy = "12" cx = "15" r = "1" ></ circle > < circle cx = "15" r = "1" cy = "5" ></ circle > < circle cy = "19" r = "1" cx = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;