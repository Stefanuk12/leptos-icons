use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 12H3" ></ path > < path d = "M16 6H3" ></ path > < path d = "M10 18H3" ></ path > < path d = "M21 6v10a2 2 0 0 1-2 2h-5" ></ path > < path d = "m16 16-2 2 2 2" ></ path > < / > } } pub const LUCIDE_LIST_END : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;