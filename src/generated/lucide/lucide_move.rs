use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "5 9 2 12 5 15" ></ polyline > < polyline points = "9 5 12 2 15 5" ></ polyline > < polyline points = "15 19 12 22 9 19" ></ polyline > < polyline points = "19 9 22 12 19 15" ></ polyline > < line y2 = "12" y1 = "12" x2 = "22" x1 = "2" ></ line > < line x1 = "12" y1 = "2" x2 = "12" y2 = "22" ></ line > < / > } } pub const LUCIDE_MOVE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;