use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 7 4 4 20 4 20 7" ></ polyline > < line x1 = "9" y2 = "20" y1 = "20" x2 = "15" ></ line > < line y2 = "20" x1 = "12" y1 = "4" x2 = "12" ></ line > < / > } } pub const LUCIDE_TYPE : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;