use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 3 21 3 21 9" ></ polyline > < polyline points = "9 21 3 21 3 15" ></ polyline > < line x1 = "21" y2 = "10" y1 = "3" x2 = "14" ></ line > < line x1 = "3" y1 = "21" x2 = "10" y2 = "14" ></ line > < / > } } pub const LUCIDE_MAXIMIZE_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;