use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "8 18 12 22 16 18" ></ polyline > < polyline points = "8 6 12 2 16 6" ></ polyline > < line x1 = "12" y1 = "2" x2 = "12" y2 = "22" ></ line > < / > } } pub const LucideMoveVertical : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;