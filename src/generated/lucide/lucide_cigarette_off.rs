use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y1 = "2" x2 = "22" y2 = "22" ></ line > < path d = "M12 12H2v4h14" ></ path > < path d = "M22 12v4" ></ path > < path d = "M18 12h-.5" ></ path > < path d = "M7 12v4" ></ path > < path d = "M18 8c0-2.5-2-2.5-2-5" ></ path > < path d = "M22 8c0-2.5-2-2.5-2-5" ></ path > < / > } } pub const LUCIDE_CIGARETTE_OFF : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;