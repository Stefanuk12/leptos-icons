use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 12H3" ></ path > < path d = "M16 6H3" ></ path > < path d = "M16 18H3" ></ path > < path d = "m19 10-4 4" ></ path > < path d = "m15 10 4 4" ></ path > < / > } } pub const LUCIDE_LIST_X : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;