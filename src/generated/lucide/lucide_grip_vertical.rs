use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "9" r = "1" ></ circle > < circle cx = "9" cy = "5" r = "1" ></ circle > < circle cy = "19" r = "1" cx = "9" ></ circle > < circle cx = "15" r = "1" cy = "12" ></ circle > < circle cx = "15" cy = "5" r = "1" ></ circle > < circle cy = "19" cx = "15" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;