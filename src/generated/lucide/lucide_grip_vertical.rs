use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "1" cx = "9" ></ circle > < circle cy = "5" r = "1" cx = "9" ></ circle > < circle r = "1" cx = "9" cy = "19" ></ circle > < circle cx = "15" cy = "12" r = "1" ></ circle > < circle cy = "5" r = "1" cx = "15" ></ circle > < circle cx = "15" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;