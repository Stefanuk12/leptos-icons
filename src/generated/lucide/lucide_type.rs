use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 7 4 4 20 4 20 7" ></ polyline > < line x1 = "9" x2 = "15" y2 = "20" y1 = "20" ></ line > < line x2 = "12" y1 = "4" y2 = "20" x1 = "12" ></ line > < / > } } pub const LUCIDE_TYPE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;