use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 7 4 4 20 4 20 7" ></ polyline > < line x2 = "15" y1 = "20" y2 = "20" x1 = "9" ></ line > < line x1 = "12" x2 = "12" y2 = "20" y1 = "4" ></ line > < / > } } pub const LUCIDE_TYPE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24")] } ;