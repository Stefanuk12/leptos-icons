use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "5 11 5 5 11 5" ></ polyline > < polyline points = "19 13 19 19 13 19" ></ polyline > < line y1 = "5" x1 = "5" x2 = "19" y2 = "19" ></ line > < / > } } pub const LUCIDE_MOVE_DIAGONAL_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;