use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v4" ></ path > < path d = "m16.2 7.8 2.9-2.9" ></ path > < path d = "M18 12h4" ></ path > < path d = "m16.2 16.2 2.9 2.9" ></ path > < path d = "M12 18v4" ></ path > < path d = "m4.9 19.1 2.9-2.9" ></ path > < path d = "M2 12h4" ></ path > < path d = "m4.9 4.9 2.9 2.9" ></ path > < / > } } pub const LUCIDE_LOADER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;