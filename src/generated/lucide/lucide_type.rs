use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 7 4 4 20 4 20 7" ></ polyline > < line x2 = "15" y2 = "20" y1 = "20" x1 = "9" ></ line > < line y2 = "20" y1 = "4" x2 = "12" x1 = "12" ></ line > < / > } } pub const LUCIDE_TYPE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2")] } ;