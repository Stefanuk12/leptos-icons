use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 12h8" ></ path > < path d = "M13 18h8" ></ path > < path d = "M13 6h8" ></ path > < path d = "M3 12h1" ></ path > < path d = "M3 18h1" ></ path > < path d = "M3 6h1" ></ path > < path d = "M8 12h1" ></ path > < path d = "M8 18h1" ></ path > < path d = "M8 6h1" ></ path > < / > } } pub const LUCIDE_LOGS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;