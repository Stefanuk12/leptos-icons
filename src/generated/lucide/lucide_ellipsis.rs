use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12" cy = "12" ></ circle > < circle cy = "12" r = "1" cx = "19" ></ circle > < circle r = "1" cx = "5" cy = "12" ></ circle > < / > } } pub const LUCIDE_ELLIPSIS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;