use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "22" x1 = "2" y1 = "2" y2 = "22" ></ line > < path d = "M12 12H2v4h14" ></ path > < path d = "M22 12v4" ></ path > < path d = "M18 12h-.5" ></ path > < path d = "M7 12v4" ></ path > < path d = "M18 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M22 8c0-2.5-2-2.5-2-5" ></ path > < / > } } pub const LUCIDE_CIGARETTE_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;