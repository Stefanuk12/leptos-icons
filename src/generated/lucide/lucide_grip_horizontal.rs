use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "9" r = "1" cx = "12" ></ circle > < circle r = "1" cx = "19" cy = "9" ></ circle > < circle cy = "9" r = "1" cx = "5" ></ circle > < circle r = "1" cx = "12" cy = "15" ></ circle > < circle cy = "15" cx = "19" r = "1" ></ circle > < circle cy = "15" r = "1" cx = "5" ></ circle > < / > } } pub const LucideGripHorizontal : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;