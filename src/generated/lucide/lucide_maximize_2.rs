use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 3 21 3 21 9" ></ polyline > < polyline points = "9 21 3 21 3 15" ></ polyline > < line y2 = "10" y1 = "3" x1 = "21" x2 = "14" ></ line > < line y1 = "21" x2 = "10" y2 = "14" x1 = "3" ></ line > < / > } } pub const LUCIDE_MAXIMIZE_2 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;