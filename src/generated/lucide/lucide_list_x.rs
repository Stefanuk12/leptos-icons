use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 12H3" ></ path > < path d = "M16 6H3" ></ path > < path d = "M16 18H3" ></ path > < path d = "m19 10-4 4" ></ path > < path d = "m15 10 4 4" ></ path > < / > } } pub const LUCIDE_LIST_X : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;