use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 18H3" ></ path > < path d = "m15 18 2 2 4-4" ></ path > < path d = "M16 12H3" ></ path > < path d = "M16 6H3" ></ path > < / > } } pub const LUCIDE_LIST_CHECK : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;