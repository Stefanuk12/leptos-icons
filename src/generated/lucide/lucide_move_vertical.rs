use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "8 18 12 22 16 18" ></ polyline > < polyline points = "8 6 12 2 16 6" ></ polyline > < line x1 = "12" x2 = "12" y1 = "2" y2 = "22" ></ line > < / > } } pub const LUCIDE_MOVE_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;