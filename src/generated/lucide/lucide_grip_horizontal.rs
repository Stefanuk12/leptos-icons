use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "9" r = "1" ></ circle > < circle cy = "9" r = "1" cx = "19" ></ circle > < circle cy = "9" r = "1" cx = "5" ></ circle > < circle cy = "15" r = "1" cx = "12" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle r = "1" cx = "5" cy = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;