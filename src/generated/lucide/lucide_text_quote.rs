use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 6H3" ></ path > < path d = "M21 12H8" ></ path > < path d = "M21 18H8" ></ path > < path d = "M3 12v6" ></ path > < / > } } pub const LUCIDE_TEXT_QUOTE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round")] } ;