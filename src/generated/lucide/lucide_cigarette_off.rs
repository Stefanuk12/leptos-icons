use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h13" ></ path > < path d = "M18 8c0-2.5-2-2.5-2-5" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M21 12a1 1 0 0 1 1 1v2a1 1 0 0 1-.5.866" ></ path > < path d = "M22 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M7 12v4" ></ path > < / > } } pub const LUCIDE_CIGARETTE_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24")] } ;