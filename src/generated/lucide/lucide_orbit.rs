use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "12" cx = "12" ></ circle > < circle cx = "19" cy = "5" r = "2" ></ circle > < circle cx = "5" r = "2" cy = "19" ></ circle > < path d = "M10.4 21.9a10 10 0 0 0 9.941-15.416" ></ path > < path d = "M13.5 2.1a10 10 0 0 0-9.841 15.416" ></ path > < / > } } pub const LUCIDE_ORBIT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;