use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 14 10 14 10 20" ></ polyline > < polyline points = "20 10 14 10 14 4" ></ polyline > < line x1 = "14" y2 = "3" x2 = "21" y1 = "10" ></ line > < line x1 = "3" x2 = "10" y2 = "14" y1 = "21" ></ line > < / > } } pub const LUCIDE_MINIMIZE_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;