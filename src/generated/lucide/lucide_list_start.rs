use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 12H3" ></ path > < path d = "M16 18H3" ></ path > < path d = "M10 6H3" ></ path > < path d = "M21 18V8a2 2 0 0 0-2-2h-5" ></ path > < path d = "m16 8-2-2 2-2" ></ path > < / > } } pub const LUCIDE_LIST_START : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;