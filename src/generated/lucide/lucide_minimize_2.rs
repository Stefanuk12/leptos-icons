use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 14 10 14 10 20" ></ polyline > < polyline points = "20 10 14 10 14 4" ></ polyline > < line x1 = "14" y1 = "10" x2 = "21" y2 = "3" ></ line > < line y2 = "14" x1 = "3" y1 = "21" x2 = "10" ></ line > < / > } } pub const LUCIDE_MINIMIZE_2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;