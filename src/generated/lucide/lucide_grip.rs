use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "5" ></ circle > < circle cx = "19" cy = "5" r = "1" ></ circle > < circle cx = "5" cy = "5" r = "1" ></ circle > < circle cx = "12" cy = "12" r = "1" ></ circle > < circle cy = "12" cx = "19" r = "1" ></ circle > < circle cx = "5" r = "1" cy = "12" ></ circle > < circle cy = "19" r = "1" cx = "12" ></ circle > < circle cy = "19" cx = "19" r = "1" ></ circle > < circle r = "1" cy = "19" cx = "5" ></ circle > < / > } } pub const LucideGrip : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;