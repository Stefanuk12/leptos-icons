use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < circle r = "6" cx = "12" cy = "12" ></ circle > < circle cy = "12" cx = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_TARGET : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor")] } ;