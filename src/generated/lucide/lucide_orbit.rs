use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "3" cy = "12" ></ circle > < circle r = "2" cx = "19" cy = "5" ></ circle > < circle cy = "19" r = "2" cx = "5" ></ circle > < path d = "M10.4 21.9a10 10 0 0 0 9.941-15.416" ></ path > < path d = "M13.5 2.1a10 10 0 0 0-9.841 15.416" ></ path > < / > } } pub const LUCIDE_ORBIT : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;