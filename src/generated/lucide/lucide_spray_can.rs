use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3h.01" ></ path > < path d = "M7 5h.01" ></ path > < path d = "M11 7h.01" ></ path > < path d = "M3 7h.01" ></ path > < path d = "M7 9h.01" ></ path > < path d = "M3 11h.01" ></ path > < rect x = "15" width = "4" y = "5" height = "4" ></ rect > < path d = "m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2" ></ path > < path d = "m13 14 8-2" ></ path > < path d = "m13 19 8-2" ></ path > < / > } } pub const LUCIDE_SPRAY_CAN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;