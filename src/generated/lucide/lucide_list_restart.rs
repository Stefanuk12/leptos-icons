use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 6H3" ></ path > < path d = "M7 12H3" ></ path > < path d = "M7 18H3" ></ path > < path d = "M12 18a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L11 14" ></ path > < path d = "M11 10v4h4" ></ path > < / > } } pub const LUCIDE_LIST_RESTART : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;