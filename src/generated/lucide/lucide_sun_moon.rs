use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 8a2.83 2.83 0 0 0 4 4 4 4 0 1 1-4-4" ></ path > < path d = "M12 2v2" ></ path > < path d = "M12 20v2" ></ path > < path d = "m4.9 4.9 1.4 1.4" ></ path > < path d = "m17.7 17.7 1.4 1.4" ></ path > < path d = "M2 12h2" ></ path > < path d = "M20 12h2" ></ path > < path d = "m6.3 17.7-1.4 1.4" ></ path > < path d = "m19.1 4.9-1.4 1.4" ></ path > < / > } } pub const LUCIDE_SUN_MOON : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;