use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 3 21 3 21 9" ></ polyline > < polyline points = "9 21 3 21 3 15" ></ polyline > < line y1 = "3" y2 = "10" x1 = "21" x2 = "14" ></ line > < line x2 = "10" x1 = "3" y1 = "21" y2 = "14" ></ line > < / > } } pub const LUCIDE_MAXIMIZE_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;