use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < path d = "m4.93 4.93 4.24 4.24" ></ path > < path d = "m14.83 9.17 4.24-4.24" ></ path > < path d = "m14.83 14.83 4.24 4.24" ></ path > < path d = "m9.17 14.83-4.24 4.24" ></ path > < circle cy = "12" r = "4" cx = "12" ></ circle > < / > } } pub const LUCIDE_LIFE_BUOY : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;