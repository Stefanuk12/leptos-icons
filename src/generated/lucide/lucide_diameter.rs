use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "19" cx = "19" r = "2" ></ circle > < circle r = "2" cx = "5" cy = "5" ></ circle > < path d = "M6.48 3.66a10 10 0 0 1 13.86 13.86" ></ path > < path d = "m6.41 6.41 11.18 11.18" ></ path > < path d = "M3.66 6.48a10 10 0 0 0 13.86 13.86" ></ path > < / > } } pub const LUCIDE_DIAMETER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;