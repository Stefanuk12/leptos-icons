use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "1" ></ circle > < circle cy = "5" r = "1" cx = "12" ></ circle > < circle cy = "19" cx = "12" r = "1" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS_VERTICAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;