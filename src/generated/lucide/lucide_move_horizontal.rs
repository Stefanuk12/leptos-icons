use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "18 8 22 12 18 16" ></ polyline > < polyline points = "6 8 2 12 6 16" ></ polyline > < line y1 = "12" x2 = "22" y2 = "12" x1 = "2" ></ line > < / > } } pub const LUCIDE_MOVE_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round")] } ;