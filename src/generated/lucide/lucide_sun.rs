use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "4" cx = "12" cy = "12" ></ circle > < path d = "M12 2v2" ></ path > < path d = "M12 20v2" ></ path > < path d = "m4.93 4.93 1.41 1.41" ></ path > < path d = "m17.66 17.66 1.41 1.41" ></ path > < path d = "M2 12h2" ></ path > < path d = "M20 12h2" ></ path > < path d = "m6.34 17.66-1.41 1.41" ></ path > < path d = "m19.07 4.93-1.41 1.41" ></ path > < / > } } pub const LUCIDE_SUN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;