use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "13 5 19 5 19 11" ></ polyline > < polyline points = "11 19 5 19 5 13" ></ polyline > < line x1 = "19" y1 = "5" x2 = "5" y2 = "19" ></ line > < / > } } pub const LUCIDE_MOVE_DIAGONAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;