use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 4h3a2 2 0 0 1 2 2v14" ></ path > < path d = "M2 20h3" ></ path > < path d = "M13 20h9" ></ path > < path d = "M10 12v.01" ></ path > < path d = "M13 4.562v16.157a1 1 0 0 1-1.242.97L5 20V5.562a2 2 0 0 1 1.515-1.94l4-1A2 2 0 0 1 13 4.561Z" ></ path > < / > } } pub const LUCIDE_DOOR_OPEN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;