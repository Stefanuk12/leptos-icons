use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "5 11 5 5 11 5" ></ polyline > < polyline points = "19 13 19 19 13 19" ></ polyline > < line x1 = "5" x2 = "19" y2 = "19" y1 = "5" ></ line > < / > } } pub const LUCIDE_MOVE_DIAGONAL_2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;