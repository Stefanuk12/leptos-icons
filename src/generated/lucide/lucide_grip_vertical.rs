use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" cy = "12" r = "1" ></ circle > < circle cy = "5" r = "1" cx = "9" ></ circle > < circle cy = "19" r = "1" cx = "9" ></ circle > < circle cy = "12" r = "1" cx = "15" ></ circle > < circle cx = "15" cy = "5" r = "1" ></ circle > < circle cy = "19" r = "1" cx = "15" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;