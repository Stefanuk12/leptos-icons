use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" cy = "12" r = "1" ></ circle > < circle cy = "5" r = "1" cx = "9" ></ circle > < circle cy = "19" cx = "9" r = "1" ></ circle > < circle r = "1" cx = "15" cy = "12" ></ circle > < circle r = "1" cx = "15" cy = "5" ></ circle > < circle cx = "15" cy = "19" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;