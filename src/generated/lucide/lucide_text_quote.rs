use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 6H3" ></ path > < path d = "M21 12H8" ></ path > < path d = "M21 18H8" ></ path > < path d = "M3 12v6" ></ path > < / > } } pub const LUCIDE_TEXT_QUOTE : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;