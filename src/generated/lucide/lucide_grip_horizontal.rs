use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "9" cx = "12" ></ circle > < circle r = "1" cy = "9" cx = "19" ></ circle > < circle r = "1" cy = "9" cx = "5" ></ circle > < circle cy = "15" r = "1" cx = "12" ></ circle > < circle cx = "19" cy = "15" r = "1" ></ circle > < circle cy = "15" cx = "5" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;