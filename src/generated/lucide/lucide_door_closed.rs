use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 20V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14" ></ path > < path d = "M2 20h20" ></ path > < path d = "M14 12v.01" ></ path > < / > } } pub const LUCIDE_DOOR_CLOSED : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;