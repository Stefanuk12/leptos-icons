use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 12H2v4h16" ></ path > < path d = "M22 12v4" ></ path > < path d = "M7 12v4" ></ path > < path d = "M18 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M22 8c0-2.5-2-2.5-2-5" ></ path > < / > } } pub const LUCIDE_CIGARETTE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;