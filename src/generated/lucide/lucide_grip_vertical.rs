use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" r = "1" cy = "12" ></ circle > < circle cy = "5" cx = "9" r = "1" ></ circle > < circle cy = "19" cx = "9" r = "1" ></ circle > < circle cx = "15" r = "1" cy = "12" ></ circle > < circle cy = "5" cx = "15" r = "1" ></ circle > < circle cy = "19" r = "1" cx = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;