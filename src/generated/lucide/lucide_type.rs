use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 7 4 4 20 4 20 7" ></ polyline > < line x1 = "9" y1 = "20" y2 = "20" x2 = "15" ></ line > < line y1 = "4" x1 = "12" y2 = "20" x2 = "12" ></ line > < / > } } pub const LUCIDE_TYPE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24")] } ;