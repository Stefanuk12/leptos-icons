use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 6H3" ></ path > < path d = "M21 12H8" ></ path > < path d = "M21 18H8" ></ path > < path d = "M3 12v6" ></ path > < / > } } pub const LUCIDE_TEXT_QUOTE : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;