use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 12H3" ></ path > < path d = "M16 6H3" ></ path > < path d = "M16 18H3" ></ path > < path d = "M18 9v6" ></ path > < path d = "M21 12h-6" ></ path > < / > } } pub const LUCIDE_LIST_PLUS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;