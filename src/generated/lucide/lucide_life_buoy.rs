use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cy = "12" cx = "12" ></ circle > < path d = "m4.93 4.93 4.24 4.24" ></ path > < path d = "m14.83 9.17 4.24-4.24" ></ path > < path d = "m14.83 14.83 4.24 4.24" ></ path > < path d = "m9.17 14.83-4.24 4.24" ></ path > < circle r = "4" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_LIFE_BUOY : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24")] } ;