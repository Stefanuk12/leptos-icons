use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 20V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14" ></ path > < path d = "M2 20h20" ></ path > < path d = "M14 12v.01" ></ path > < / > } } pub const LUCIDE_DOOR_CLOSED : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;