use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "12" cx = "12" ></ circle > < circle cy = "5" r = "1" cx = "12" ></ circle > < circle cy = "19" r = "1" cx = "12" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS_VERTICAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;