use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 8a2.83 2.83 0 0 0 4 4 4 4 0 1 1-4-4" ></ path > < path d = "M12 2v2" ></ path > < path d = "M12 20v2" ></ path > < path d = "m4.9 4.9 1.4 1.4" ></ path > < path d = "m17.7 17.7 1.4 1.4" ></ path > < path d = "M2 12h2" ></ path > < path d = "M20 12h2" ></ path > < path d = "m6.3 17.7-1.4 1.4" ></ path > < path d = "m19.1 4.9-1.4 1.4" ></ path > < / > } } pub const LucideSunMoon : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;