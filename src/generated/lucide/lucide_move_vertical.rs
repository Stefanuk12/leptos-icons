use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "8 18 12 22 16 18" ></ polyline > < polyline points = "8 6 12 2 16 6" ></ polyline > < line x1 = "12" x2 = "12" y2 = "22" y1 = "2" ></ line > < / > } } pub const LUCIDE_MOVE_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;