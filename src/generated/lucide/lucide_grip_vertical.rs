use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" r = "1" cy = "12" ></ circle > < circle cy = "5" r = "1" cx = "9" ></ circle > < circle r = "1" cy = "19" cx = "9" ></ circle > < circle cy = "12" r = "1" cx = "15" ></ circle > < circle r = "1" cx = "15" cy = "5" ></ circle > < circle cy = "19" cx = "15" r = "1" ></ circle > < / > } } pub const LUCIDE_GRIP_VERTICAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;