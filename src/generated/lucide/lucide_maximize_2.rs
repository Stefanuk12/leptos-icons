use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 3 21 3 21 9" ></ polyline > < polyline points = "9 21 3 21 3 15" ></ polyline > < line y1 = "3" x2 = "14" y2 = "10" x1 = "21" ></ line > < line x1 = "3" y1 = "21" x2 = "10" y2 = "14" ></ line > < / > } } pub const LUCIDE_MAXIMIZE_2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;