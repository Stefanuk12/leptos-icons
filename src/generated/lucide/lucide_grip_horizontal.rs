use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "9" cx = "12" r = "1" ></ circle > < circle cy = "9" r = "1" cx = "19" ></ circle > < circle cy = "9" r = "1" cx = "5" ></ circle > < circle r = "1" cx = "12" cy = "15" ></ circle > < circle cy = "15" cx = "19" r = "1" ></ circle > < circle r = "1" cx = "5" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;