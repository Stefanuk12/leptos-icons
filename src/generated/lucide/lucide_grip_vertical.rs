use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "1" cx = "9" ></ circle > < circle cx = "9" r = "1" cy = "5" ></ circle > < circle cx = "9" r = "1" cy = "19" ></ circle > < circle cx = "15" cy = "12" r = "1" ></ circle > < circle r = "1" cy = "5" cx = "15" ></ circle > < circle r = "1" cx = "15" cy = "19" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;