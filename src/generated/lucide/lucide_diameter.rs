use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "19" r = "2" cy = "19" ></ circle > < circle cy = "5" cx = "5" r = "2" ></ circle > < path d = "M6.48 3.66a10 10 0 0 1 13.86 13.86" ></ path > < path d = "m6.41 6.41 11.18 11.18" ></ path > < path d = "M3.66 6.48a10 10 0 0 0 13.86 13.86" ></ path > < / > } } pub const LUCIDE_DIAMETER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24")] } ;