use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "4" ></ circle > < path d = "M12 2v2" ></ path > < path d = "M12 20v2" ></ path > < path d = "m4.93 4.93 1.41 1.41" ></ path > < path d = "m17.66 17.66 1.41 1.41" ></ path > < path d = "M2 12h2" ></ path > < path d = "M20 12h2" ></ path > < path d = "m6.34 17.66-1.41 1.41" ></ path > < path d = "m19.07 4.93-1.41 1.41" ></ path > < / > } } pub const LUCIDE_SUN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24")] } ;