use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "13 5 19 5 19 11" ></ polyline > < polyline points = "11 19 5 19 5 13" ></ polyline > < line x2 = "5" y2 = "19" y1 = "5" x1 = "19" ></ line > < / > } } pub const LUCIDE_MOVE_DIAGONAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;